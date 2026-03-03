"""
Baseline UT: show queue counters (Cmd #53)

Source:
  - sonic-utilities UT File : queue_counter_test.py
  - sonic-utilities Test Func: test_queue_counters
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #53

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show queue counters"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Ethernet0:0": {
        "Counter/pkts": "0",
        "Counter/bytes": "0",
        "Drop/pkts": "0",
        "Drop/bytes": "0"
    },
    "Ethernet0:1": {
        "Counter/pkts": "60",
        "Counter/bytes": "43",
        "Drop/pkts": "39",
        "Drop/bytes": "1"
    },
    "Ethernet0:2": {
        "Counter/pkts": "82",
        "Counter/bytes": "7",
        "Drop/pkts": "39",
        "Drop/bytes": "21"
    },
    "Ethernet0:3": {
        "Counter/pkts": "52",
        "Counter/bytes": "70",
        "Drop/pkts": "19",
        "Drop/bytes": "76"
    },
    "Ethernet0:4": {
        "Counter/pkts": "11",
        "Counter/bytes": "59",
        "Drop/pkts": "12",
        "Drop/bytes": "94"
    },
    "Ethernet0:5": {
        "Counter/pkts": "36",
        "Counter/bytes": "62",
        "Drop/pkts": "35",
        "Drop/bytes": "40"
    },
    "Ethernet0:6": {
        "Counter/pkts": "49",
        "Counter/bytes": "91",
        "Drop/pkts": "2",
        "Drop/bytes": "88"
    },
    "Ethernet0:7": {
        "Counter/pkts": "33",
        "Counter/bytes": "17",
        "Drop/pkts": "94",
        "Drop/bytes": "74"
    },
    "Ethernet0:8": {
        "Counter/pkts": "40",
        "Counter/bytes": "71",
        "Drop/pkts": "95",
        "Drop/bytes": "33"
    },
    "Ethernet0:9": {
        "Counter/pkts": "54",
        "Counter/bytes": "8",
        "Drop/pkts": "93",
        "Drop/bytes": "78"
    },
    "Ethernet0:10": {
        "Counter/pkts": "83",
        "Counter/bytes": "96",
        "Drop/pkts": "74",
        "Drop/bytes": "9"
    },
    "Ethernet0:11": {
        "Counter/pkts": "15",
        "Counter/bytes": "60",
        "Drop/pkts": "61",
        "Drop/bytes": "31"
    },
    "Ethernet0:12": {
        "Counter/pkts": "45",
        "Counter/bytes": "52",
        "Drop/pkts": "82",
        "Drop/bytes": "94"
    },
    "Ethernet0:13": {
        "Counter/pkts": "55",
        "Counter/bytes": "88",
        "Drop/pkts": "89",
        "Drop/bytes": "52"
    },
    "Ethernet0:14": {
        "Counter/pkts": "14",
        "Counter/bytes": "70",
        "Drop/pkts": "95",
        "Drop/bytes": "79"
    },
    "Ethernet0:15": {
        "Counter/pkts": "68",
        "Counter/bytes": "60",
        "Drop/pkts": "66",
        "Drop/bytes": "81"
    },
    "Ethernet0:16": {
        "Counter/pkts": "63",
        "Counter/bytes": "4",
        "Drop/pkts": "48",
        "Drop/bytes": "76"
    },
    "Ethernet0:17": {
        "Counter/pkts": "41",
        "Counter/bytes": "73",
        "Drop/pkts": "77",
        "Drop/bytes": "74"
    },
    "Ethernet0:18": {
        "Counter/pkts": "60",
        "Counter/bytes": "21",
        "Drop/pkts": "56",
        "Drop/bytes": "54"
    },
    "Ethernet0:19": {
        "Counter/pkts": "57",
        "Counter/bytes": "31",
        "Drop/pkts": "12",
        "Drop/bytes": "39"
    },
    "Ethernet0:20": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet0:21": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet0:22": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet0:23": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet0:24": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet0:25": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet0:26": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet0:27": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet0:28": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet0:29": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet4:0": {
        "Counter/pkts": "41",
        "Counter/bytes": "96",
        "Drop/pkts": "70",
        "Drop/bytes": "98"
    },
    "Ethernet4:1": {
        "Counter/pkts": "18",
        "Counter/bytes": "49",
        "Drop/pkts": "63",
        "Drop/bytes": "36"
    },
    "Ethernet4:2": {
        "Counter/pkts": "99",
        "Counter/bytes": "90",
        "Drop/pkts": "3",
        "Drop/bytes": "15"
    },
    "Ethernet4:3": {
        "Counter/pkts": "60",
        "Counter/bytes": "89",
        "Drop/pkts": "48",
        "Drop/bytes": "41"
    },
    "Ethernet4:4": {
        "Counter/pkts": "8",
        "Counter/bytes": "84",
        "Drop/pkts": "82",
        "Drop/bytes": "94"
    },
    "Ethernet4:5": {
        "Counter/pkts": "83",
        "Counter/bytes": "15",
        "Drop/pkts": "75",
        "Drop/bytes": "92"
    },
    "Ethernet4:6": {
        "Counter/pkts": "84",
        "Counter/bytes": "26",
        "Drop/pkts": "50",
        "Drop/bytes": "71"
    },
    "Ethernet4:7": {
        "Counter/pkts": "27",
        "Counter/bytes": "19",
        "Drop/pkts": "49",
        "Drop/bytes": "80"
    },
    "Ethernet4:8": {
        "Counter/pkts": "13",
        "Counter/bytes": "89",
        "Drop/pkts": "13",
        "Drop/bytes": "33"
    },
    "Ethernet4:9": {
        "Counter/pkts": "43",
        "Counter/bytes": "48",
        "Drop/pkts": "86",
        "Drop/bytes": "31"
    },
    "Ethernet4:10": {
        "Counter/pkts": "50",
        "Counter/bytes": "1",
        "Drop/pkts": "57",
        "Drop/bytes": "82"
    },
    "Ethernet4:11": {
        "Counter/pkts": "67",
        "Counter/bytes": "99",
        "Drop/pkts": "84",
        "Drop/bytes": "59"
    },
    "Ethernet4:12": {
        "Counter/pkts": "4",
        "Counter/bytes": "58",
        "Drop/pkts": "27",
        "Drop/bytes": "5"
    },
    "Ethernet4:13": {
        "Counter/pkts": "74",
        "Counter/bytes": "5",
        "Drop/pkts": "57",
        "Drop/bytes": "39"
    },
    "Ethernet4:14": {
        "Counter/pkts": "21",
        "Counter/bytes": "59",
        "Drop/pkts": "4",
        "Drop/bytes": "14"
    },
    "Ethernet4:15": {
        "Counter/pkts": "24",
        "Counter/bytes": "61",
        "Drop/pkts": "19",
        "Drop/bytes": "53"
    },
    "Ethernet4:16": {
        "Counter/pkts": "51",
        "Counter/bytes": "15",
        "Drop/pkts": "15",
        "Drop/bytes": "32"
    },
    "Ethernet4:17": {
        "Counter/pkts": "98",
        "Counter/bytes": "18",
        "Drop/pkts": "23",
        "Drop/bytes": "15"
    },
    "Ethernet4:18": {
        "Counter/pkts": "41",
        "Counter/bytes": "34",
        "Drop/pkts": "9",
        "Drop/bytes": "57"
    },
    "Ethernet4:19": {
        "Counter/pkts": "57",
        "Counter/bytes": "7",
        "Drop/pkts": "18",
        "Drop/bytes": "99"
    },
    "Ethernet4:20": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet4:21": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet4:22": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet4:23": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet4:24": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet4:25": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet4:26": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet4:27": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet4:28": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet4:29": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet8:0": {
        "Counter/pkts": "0",
        "Counter/bytes": "0",
        "Drop/pkts": "0",
        "Drop/bytes": "0"
    },
    "Ethernet8:1": {
        "Counter/pkts": "38",
        "Counter/bytes": "17",
        "Drop/pkts": "68",
        "Drop/bytes": "91"
    },
    "Ethernet8:2": {
        "Counter/pkts": "16",
        "Counter/bytes": "65",
        "Drop/pkts": "79",
        "Drop/bytes": "51"
    },
    "Ethernet8:3": {
        "Counter/pkts": "11",
        "Counter/bytes": "97",
        "Drop/pkts": "63",
        "Drop/bytes": "72"
    },
    "Ethernet8:4": {
        "Counter/pkts": "54",
        "Counter/bytes": "89",
        "Drop/pkts": "62",
        "Drop/bytes": "62"
    },
    "Ethernet8:5": {
        "Counter/pkts": "13",
        "Counter/bytes": "84",
        "Drop/pkts": "30",
        "Drop/bytes": "59"
    },
    "Ethernet8:6": {
        "Counter/pkts": "49",
        "Counter/bytes": "67",
        "Drop/pkts": "99",
        "Drop/bytes": "85"
    },
    "Ethernet8:7": {
        "Counter/pkts": "2",
        "Counter/bytes": "63",
        "Drop/pkts": "38",
        "Drop/bytes": "88"
    },
    "Ethernet8:8": {
        "Counter/pkts": "0",
        "Counter/bytes": "82",
        "Drop/pkts": "93",
        "Drop/bytes": "43"
    },
    "Ethernet8:9": {
        "Counter/pkts": "80",
        "Counter/bytes": "17",
        "Drop/pkts": "91",
        "Drop/bytes": "61"
    },
    "Ethernet8:10": {
        "Counter/pkts": "81",
        "Counter/bytes": "63",
        "Drop/pkts": "76",
        "Drop/bytes": "73"
    },
    "Ethernet8:11": {
        "Counter/pkts": "29",
        "Counter/bytes": "16",
        "Drop/pkts": "29",
        "Drop/bytes": "66"
    },
    "Ethernet8:12": {
        "Counter/pkts": "32",
        "Counter/bytes": "12",
        "Drop/pkts": "61",
        "Drop/bytes": "35"
    },
    "Ethernet8:13": {
        "Counter/pkts": "79",
        "Counter/bytes": "17",
        "Drop/pkts": "72",
        "Drop/bytes": "93"
    },
    "Ethernet8:14": {
        "Counter/pkts": "23",
        "Counter/bytes": "21",
        "Drop/pkts": "67",
        "Drop/bytes": "50"
    },
    "Ethernet8:15": {
        "Counter/pkts": "37",
        "Counter/bytes": "10",
        "Drop/pkts": "97",
        "Drop/bytes": "14"
    },
    "Ethernet8:16": {
        "Counter/pkts": "30",
        "Counter/bytes": "17",
        "Drop/pkts": "74",
        "Drop/bytes": "43"
    },
    "Ethernet8:17": {
        "Counter/pkts": "0",
        "Counter/bytes": "63",
        "Drop/pkts": "54",
        "Drop/bytes": "84"
    },
    "Ethernet8:18": {
        "Counter/pkts": "69",
        "Counter/bytes": "88",
        "Drop/pkts": "24",
        "Drop/bytes": "79"
    },
    "Ethernet8:19": {
        "Counter/pkts": "20",
        "Counter/bytes": "12",
        "Drop/pkts": "84",
        "Drop/bytes": "3"
    },
    "Ethernet8:20": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet8:21": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet8:22": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet8:23": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet8:24": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet8:25": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet8:26": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet8:27": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet8:28": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    },
    "Ethernet8:29": {
        "Counter/pkts": "N/A",
        "Counter/bytes": "N/A",
        "Drop/pkts": "N/A",
        "Drop/bytes": "N/A"
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
For namespace :
     Port    TxQ    Counter/pkts    Counter/bytes    Drop/pkts    Drop/bytes
---------  -----  --------------  ---------------  -----------  ------------
Ethernet0    UC0               0                0            0             0
Ethernet0    UC1              60               43           39             1
Ethernet0    UC2              82                7           39            21
Ethernet0    UC3              52               70           19            76
Ethernet0    UC4              11               59           12            94
Ethernet0    UC5              36               62           35            40
Ethernet0    UC6              49               91            2            88
Ethernet0    UC7              33               17           94            74
Ethernet0    UC8              40               71           95            33
Ethernet0    UC9              54                8           93            78
Ethernet0   MC10              83               96           74             9
Ethernet0   MC11              15               60           61            31
Ethernet0   MC12              45               52           82            94
Ethernet0   MC13              55               88           89            52
Ethernet0   MC14              14               70           95            79
Ethernet0   MC15              68               60           66            81
Ethernet0   MC16              63                4           48            76
Ethernet0   MC17              41               73           77            74
Ethernet0   MC18              60               21           56            54
Ethernet0   MC19              57               31           12            39
Ethernet0  ALL20             N/A              N/A          N/A           N/A
Ethernet0  ALL21             N/A              N/A          N/A           N/A
Ethernet0  ALL22             N/A              N/A          N/A           N/A
Ethernet0  ALL23             N/A              N/A          N/A           N/A
Ethernet0  ALL24             N/A              N/A          N/A           N/A
Ethernet0  ALL25             N/A              N/A          N/A           N/A
Ethernet0  ALL26             N/A              N/A          N/A           N/A
Ethernet0  ALL27             N/A              N/A          N/A           N/A
Ethernet0  ALL28             N/A              N/A          N/A           N/A
Ethernet0  ALL29             N/A              N/A          N/A           N/A

For namespace :
     Port    TxQ    Counter/pkts    Counter/bytes    Drop/pkts    Drop/bytes
---------  -----  --------------  ---------------  -----------  ------------
Ethernet4    UC0              41               96           70            98
Ethernet4    UC1              18               49           63            36
Ethernet4    UC2              99               90            3            15
Ethernet4    UC3              60               89           48            41
Ethernet4    UC4               8               84           82            94
Ethernet4    UC5              83               15           75            92
Ethernet4    UC6              84               26           50            71
Ethernet4    UC7              27               19           49            80
Ethernet4    UC8              13               89           13            33
Ethernet4    UC9              43               48           86            31
Ethernet4   MC10              50                1           57            82
Ethernet4   MC11              67               99           84            59
Ethernet4   MC12               4               58           27             5
Ethernet4   MC13              74                5           57            39
Ethernet4   MC14              21               59            4            14
Ethernet4   MC15              24               61           19            53
Ethernet4   MC16              51               15           15            32
Ethernet4   MC17              98               18           23            15
Ethernet4   MC18              41               34            9            57
Ethernet4   MC19              57                7           18            99
Ethernet4  ALL20             N/A              N/A          N/A           N/A
Ethernet4  ALL21             N/A              N/A          N/A           N/A
Ethernet4  ALL22             N/A              N/A          N/A           N/A
Ethernet4  ALL23             N/A              N/A          N/A           N/A
Ethernet4  ALL24             N/A              N/A          N/A           N/A
Ethernet4  ALL25             N/A              N/A          N/A           N/A
Ethernet4  ALL26             N/A              N/A          N/A           N/A
Ethernet4  ALL27             N/A              N/A          N/A           N/A
Ethernet4  ALL28             N/A              N/A          N/A           N/A
Ethernet4  ALL29             N/A              N/A          N/A           N/A

For namespace :
     Port    TxQ    Counter/pkts    Counter/bytes    Drop/pkts    Drop/bytes
---------  -----  --------------  ---------------  -----------  ------------
Ethernet8    UC0               0                0            0             0
Ethernet8    UC1              38               17           68            91
Ethernet8    UC2              16               65           79            51
Ethernet8    UC3              11               97           63            72
Ethernet8    UC4              54               89           62            62
Ethernet8    UC5              13               84           30            59
Ethernet8    UC6              49               67           99            85
Ethernet8    UC7               2               63           38            88
Ethernet8    UC8               0               82           93            43
Ethernet8    UC9              80               17           91            61
Ethernet8   MC10              81               63           76            73
Ethernet8   MC11              29               16           29            66
Ethernet8   MC12              32               12           61            35
Ethernet8   MC13              79               17           72            93
Ethernet8   MC14              23               21           67            50
Ethernet8   MC15              37               10           97            14
Ethernet8   MC16              30               17           74            43
Ethernet8   MC17               0               63           54            84
Ethernet8   MC18              69               88           24            79
Ethernet8   MC19              20               12           84             3
Ethernet8  ALL20             N/A              N/A          N/A           N/A
Ethernet8  ALL21             N/A              N/A          N/A           N/A
Ethernet8  ALL22             N/A              N/A          N/A           N/A
Ethernet8  ALL23             N/A              N/A          N/A           N/A
Ethernet8  ALL24             N/A              N/A          N/A           N/A
Ethernet8  ALL25             N/A              N/A          N/A           N/A
Ethernet8  ALL26             N/A              N/A          N/A           N/A
Ethernet8  ALL27             N/A              N/A          N/A           N/A
Ethernet8  ALL28             N/A              N/A          N/A           N/A
Ethernet8  ALL29             N/A              N/A          N/A           N/A"""


@pytest.mark.sonic_baseline
class TestQueueCounters:
    """
    Baseline test for 'show queue counters' (Cmd #53).

    Mirrors sonic-utilities test:
      File: queue_counter_test.py
      Func: test_queue_counters
    """

    def test_converter_output_matches_sonic_ut(self, command_parser):
        """JSON -> Converter -> CLI text should match sonic-utilities expected output."""
        actual = command_parser.parse_and_convert(CLI_COMMAND, json.dumps(JSON_INPUT))
        assert actual.strip() == EXPECTED_CLI_OUTPUT.strip(), (
            f"Converter output for '{CLI_COMMAND}' does not match "
            f"sonic-utilities baseline.\n"
            f"--- Expected ---\n{EXPECTED_CLI_OUTPUT}\n"
            f"--- Actual ---\n{actual}"
        )
