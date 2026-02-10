use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gnmi_cli_lib::converter::GnmiToCliConverter;
use gnmi_cli_lib::parser::CommandParser;
use gnmi_cli_lib::registry::create_default_registry;

fn bench_converter_creation(c: &mut Criterion) {
    c.bench_function("create_converter", |b| {
        b.iter(|| {
            let converter = GnmiToCliConverter::new();
            black_box(converter);
        });
    });
}

fn bench_parser_creation(c: &mut Criterion) {
    c.bench_function("create_parser", |b| {
        b.iter(|| {
            let parser = CommandParser::new();
            black_box(parser);
        });
    });
}

fn bench_registry_creation(c: &mut Criterion) {
    c.bench_function("create_default_registry", |b| {
        b.iter(|| {
            let registry = create_default_registry();
            black_box(registry);
        });
    });
}

fn bench_convert_interfaces_status(c: &mut Criterion) {
    let converter = GnmiToCliConverter::new();
    let json_input = serde_json::json!({
        "sonic-port:PORT_TABLE_LIST": [
            {
                "ifname": "Ethernet0",
                "admin_status": "up",
                "oper_status": "up",
                "speed": "100000",
                "mtu": "9100",
                "description": "ARISTA01T0:Ethernet0"
            },
            {
                "ifname": "Ethernet4",
                "admin_status": "up",
                "oper_status": "down",
                "speed": "25000",
                "mtu": "9100",
                "description": "ARISTA02T0:Ethernet0"
            }
        ]
    });

    c.bench_function("convert_interfaces_status", |b| {
        b.iter(|| {
            let result = converter.convert(
                black_box("interfaces_status"),
                black_box(json_input.clone()),
            );
            let _ = black_box(result);
        });
    });
}

fn bench_convert_interfaces_counters(c: &mut Criterion) {
    let converter = GnmiToCliConverter::new();
    let json_input = serde_json::json!({
        "sonic-counters:counters": {
            "Ethernet0": {
                "SAI_PORT_STAT_IF_IN_OCTETS": "100",
                "SAI_PORT_STAT_IF_IN_UCAST_PKTS": "200",
                "SAI_PORT_STAT_IF_OUT_OCTETS": "300",
                "SAI_PORT_STAT_IF_OUT_UCAST_PKTS": "400"
            }
        }
    });

    c.bench_function("convert_interfaces_counters", |b| {
        b.iter(|| {
            let result = converter.convert(
                black_box("interfaces_counters"),
                black_box(json_input.clone()),
            );
            let _ = black_box(result);
        });
    });
}

fn bench_list_supported_commands(c: &mut Criterion) {
    let converter = GnmiToCliConverter::new();
    c.bench_function("list_supported_commands", |b| {
        b.iter(|| {
            let cmds = converter.available_commands();
            black_box(cmds);
        });
    });
}

fn bench_parse_command(c: &mut Criterion) {
    let parser = CommandParser::new();
    c.bench_function("parse_show_interfaces_status", |b| {
        b.iter(|| {
            let result = parser.parse(black_box("show interfaces status"));
            black_box(result);
        });
    });
}

criterion_group!(
    benches,
    bench_converter_creation,
    bench_parser_creation,
    bench_registry_creation,
    bench_convert_interfaces_status,
    bench_convert_interfaces_counters,
    bench_list_supported_commands,
    bench_parse_command,
);
criterion_main!(benches);
