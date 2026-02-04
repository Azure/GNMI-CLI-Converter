"""
Script to merge test data files 71-98 into their corresponding base command files (1-70).

This script:
1. Maps extra files to their base command
2. Extracts the argument from the command name
3. Adds test cases with proper input_options to the base file
4. Handles special cases where commands require arguments
"""

import json
import os
import re
import glob
import sys

# Directory containing test data
TEST_DATA_DIR = r"c:\Users\shixz\OneDrive - Microsoft\WorkingFolder\AdhocAsks\gnmi_cli_lib_verifying\test_data"


def get_all_files():
    """Get all data_*.json files."""
    pattern = os.path.join(TEST_DATA_DIR, "data_*.json")
    return sorted(glob.glob(pattern))


def load_json(filepath):
    """Load JSON from file."""
    with open(filepath, 'r', encoding='utf-8') as f:
        return json.load(f)


def save_json(filepath, data):
    """Save JSON to file."""
    with open(filepath, 'w', encoding='utf-8') as f:
        json.dump(data, f, indent=2, ensure_ascii=False)


def extract_base_command(filename):
    """
    Extract the base command from a filename.
    E.g., data_11_show_interfaces_counters_detailed_Ethernet0.json 
    -> "show interfaces counters detailed"
    """
    # Remove data_XX_ prefix and .json suffix
    name = re.sub(r'^data_\d+_', '', os.path.basename(filename))
    name = name.replace('.json', '')
    
    # Convert underscores to spaces
    parts = name.split('_')
    
    # Find where the interface/argument starts
    # Common argument patterns: Ethernet*, PortChannel*, fc00*, DEFAULT*, vrf, iface, fe80, numeric
    base_parts = []
    arg_parts = []
    found_arg = False
    
    for part in parts:
        if not found_arg and re.match(r'^(Ethernet|PortChannel|fc00|DEFAULT|vrf|iface|fe80|\d+$)', part, re.IGNORECASE):
            found_arg = True
        
        if found_arg:
            arg_parts.append(part)
        else:
            base_parts.append(part)
    
    base_command = ' '.join(base_parts)
    argument = ' '.join(arg_parts) if arg_parts else ''
    
    return base_command, argument


def normalize_command(cmd):
    """Normalize command by removing extra spaces and converting to lowercase."""
    return ' '.join(cmd.lower().split())


def get_file_number(filename):
    """Extract the file number from the filename."""
    match = re.search(r'data_(\d+)_', os.path.basename(filename))
    if match:
        return int(match.group(1))
    return 999


def main():
    print("Analyzing test data files...")
    print("=" * 80)
    
    files = get_all_files()
    
    # Categorize files
    base_files = []  # Files numbered 01-70
    extra_files = []  # Files numbered 71-98
    
    for f in files:
        num = get_file_number(f)
        if num <= 70:
            base_files.append(f)
        else:
            extra_files.append(f)
    
    print(f"Found {len(base_files)} base files (01-70)")
    print(f"Found {len(extra_files)} extra files (71-98)")
    print()
    
    # Build a map of base command -> base file
    base_cmd_map = {}
    for bf in base_files:
        bf_base_cmd, _ = extract_base_command(bf)
        normalized = normalize_command(bf_base_cmd)
        base_cmd_map[normalized] = bf
    
    # Analyze extra files and create merge plan
    print("Analyzing extra files...")
    print("-" * 80)
    
    merge_plan = []
    
    for f in extra_files:
        data = load_json(f)
        command = data.get('command', 'UNKNOWN')
        base_cmd, arg = extract_base_command(f)
        
        print(f"File: {os.path.basename(f)}")
        print(f"  Command: {command}")
        print(f"  Base: {base_cmd}")
        print(f"  Arg: {arg}")
        
        # Find matching base file
        normalized = normalize_command(base_cmd)
        target = base_cmd_map.get(normalized)
        
        if target:
            print(f"  -> Merge into: {os.path.basename(target)}")
            # Calculate input_options from the actual command vs base command
            target_data = load_json(target)
            target_cmd = target_data.get('command', '')
            
            if command.lower().startswith(target_cmd.lower()):
                input_opts = command[len(target_cmd):].strip()
            else:
                input_opts = arg
            
            merge_plan.append((f, target, input_opts))
        else:
            print(f"  -> NO MATCH FOUND")
        print()
    
    print("\n" + "=" * 80)
    print("MERGE PLAN:")
    print("=" * 80)
    
    for src, dst, arg in merge_plan:
        print(f"  {os.path.basename(src)} -> {os.path.basename(dst)} (input_options: '{arg}')")
    
    print("\n" + "=" * 80)
    print(f"Total merges planned: {len(merge_plan)}")
    
    # Check for execution flag
    if "--execute" not in sys.argv:
        print("\nRun with --execute to perform the merge")
        return
    
    print("\nExecuting merge...")
    
    for src, dst, input_opts in merge_plan:
        print(f"\nMerging: {os.path.basename(src)}")
        
        src_data = load_json(src)
        dst_data = load_json(dst)
        
        # Add test cases from source to destination
        for tc in src_data.get('test_cases', []):
            # Create a new test case with proper input_options
            new_tc = {
                "input_options": input_opts if input_opts else tc.get('input_options', ''),
                "cli_file": tc.get('cli_file', ''),
                "gnmi_file": tc.get('gnmi_file', ''),
                "devices": tc.get('devices', {})
            }
            
            # Check if this input_options already exists
            existing = [t.get('input_options', '') for t in dst_data.get('test_cases', [])]
            if new_tc['input_options'] not in existing:
                dst_data['test_cases'].append(new_tc)
                print(f"  Added test case with input_options: '{new_tc['input_options']}'")
            else:
                print(f"  Skipped - input_options '{new_tc['input_options']}' already exists")
        
        # Save the updated destination file
        save_json(dst, dst_data)
    
    print("\n" + "=" * 80)
    print("Merge complete!")
    print("\nFiles that can be deleted (after verification):")
    for src, _, _ in merge_plan:
        print(f"  {os.path.basename(src)}")
    
    # Delete files if requested
    if "--delete" in sys.argv:
        print("\nDeleting merged files...")
        for src, _, _ in merge_plan:
            os.remove(src)
            print(f"  Deleted: {os.path.basename(src)}")
        print("Done!")


if __name__ == "__main__":
    main()

