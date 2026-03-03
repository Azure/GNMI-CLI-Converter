"""
Baseline UT: show queue wredcounters (Cmd #62)

Source:
  - sonic-utilities UT File : wred_queue_counter_test.py
  - sonic-utilities Test Func: test_show_wred_queue_counters
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #62

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show queue wredcounters"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Ethernet0:0": {
        "WredDrp/pkts": "0",
        "WredDrp/bytes": "0",
        "EcnMarked/pkts": "0",
        "EcnMarked/bytes": "0"
    },
    "Ethernet0:1": {
        "WredDrp/pkts": "60",
        "WredDrp/bytes": "43",
        "EcnMarked/pkts": "39",
        "EcnMarked/bytes": "1"
    },
    "Ethernet0:2": {
        "WredDrp/pkts": "82",
        "WredDrp/bytes": "7",
        "EcnMarked/pkts": "39",
        "EcnMarked/bytes": "21"
    },
    "Ethernet0:3": {
        "WredDrp/pkts": "52",
        "WredDrp/bytes": "70",
        "EcnMarked/pkts": "19",
        "EcnMarked/bytes": "76"
    },
    "Ethernet0:4": {
        "WredDrp/pkts": "11",
        "WredDrp/bytes": "59",
        "EcnMarked/pkts": "12",
        "EcnMarked/bytes": "94"
    },
    "Ethernet0:5": {
        "WredDrp/pkts": "36",
        "WredDrp/bytes": "62",
        "EcnMarked/pkts": "35",
        "EcnMarked/bytes": "40"
    },
    "Ethernet0:6": {
        "WredDrp/pkts": "49",
        "WredDrp/bytes": "91",
        "EcnMarked/pkts": "2",
        "EcnMarked/bytes": "88"
    },
    "Ethernet0:7": {
        "WredDrp/pkts": "33",
        "WredDrp/bytes": "17",
        "EcnMarked/pkts": "94",
        "EcnMarked/bytes": "74"
    },
    "Ethernet0:8": {
        "WredDrp/pkts": "40",
        "WredDrp/bytes": "71",
        "EcnMarked/pkts": "95",
        "EcnMarked/bytes": "33"
    },
    "Ethernet0:9": {
        "WredDrp/pkts": "54",
        "WredDrp/bytes": "8",
        "EcnMarked/pkts": "93",
        "EcnMarked/bytes": "78"
    },
    "Ethernet0:10": {
        "WredDrp/pkts": "83",
        "WredDrp/bytes": "96",
        "EcnMarked/pkts": "74",
        "EcnMarked/bytes": "9"
    },
    "Ethernet0:11": {
        "WredDrp/pkts": "15",
        "WredDrp/bytes": "60",
        "EcnMarked/pkts": "61",
        "EcnMarked/bytes": "31"
    },
    "Ethernet0:12": {
        "WredDrp/pkts": "45",
        "WredDrp/bytes": "52",
        "EcnMarked/pkts": "82",
        "EcnMarked/bytes": "94"
    },
    "Ethernet0:13": {
        "WredDrp/pkts": "55",
        "WredDrp/bytes": "88",
        "EcnMarked/pkts": "89",
        "EcnMarked/bytes": "52"
    },
    "Ethernet0:14": {
        "WredDrp/pkts": "14",
        "WredDrp/bytes": "70",
        "EcnMarked/pkts": "95",
        "EcnMarked/bytes": "79"
    },
    "Ethernet0:15": {
        "WredDrp/pkts": "68",
        "WredDrp/bytes": "60",
        "EcnMarked/pkts": "66",
        "EcnMarked/bytes": "81"
    },
    "Ethernet0:16": {
        "WredDrp/pkts": "63",
        "WredDrp/bytes": "4",
        "EcnMarked/pkts": "48",
        "EcnMarked/bytes": "76"
    },
    "Ethernet0:17": {
        "WredDrp/pkts": "41",
        "WredDrp/bytes": "73",
        "EcnMarked/pkts": "77",
        "EcnMarked/bytes": "74"
    },
    "Ethernet0:18": {
        "WredDrp/pkts": "60",
        "WredDrp/bytes": "21",
        "EcnMarked/pkts": "56",
        "EcnMarked/bytes": "54"
    },
    "Ethernet0:19": {
        "WredDrp/pkts": "57",
        "WredDrp/bytes": "31",
        "EcnMarked/pkts": "12",
        "EcnMarked/bytes": "39"
    },
    "Ethernet0:20": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet0:21": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet0:22": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet0:23": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet0:24": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet0:25": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet0:26": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet0:27": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet0:28": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet0:29": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet4:0": {
        "WredDrp/pkts": "41",
        "WredDrp/bytes": "96",
        "EcnMarked/pkts": "70",
        "EcnMarked/bytes": "98"
    },
    "Ethernet4:1": {
        "WredDrp/pkts": "18",
        "WredDrp/bytes": "49",
        "EcnMarked/pkts": "63",
        "EcnMarked/bytes": "36"
    },
    "Ethernet4:2": {
        "WredDrp/pkts": "99",
        "WredDrp/bytes": "90",
        "EcnMarked/pkts": "3",
        "EcnMarked/bytes": "15"
    },
    "Ethernet4:3": {
        "WredDrp/pkts": "60",
        "WredDrp/bytes": "89",
        "EcnMarked/pkts": "48",
        "EcnMarked/bytes": "41"
    },
    "Ethernet4:4": {
        "WredDrp/pkts": "8",
        "WredDrp/bytes": "84",
        "EcnMarked/pkts": "82",
        "EcnMarked/bytes": "94"
    },
    "Ethernet4:5": {
        "WredDrp/pkts": "83",
        "WredDrp/bytes": "15",
        "EcnMarked/pkts": "75",
        "EcnMarked/bytes": "92"
    },
    "Ethernet4:6": {
        "WredDrp/pkts": "84",
        "WredDrp/bytes": "26",
        "EcnMarked/pkts": "50",
        "EcnMarked/bytes": "71"
    },
    "Ethernet4:7": {
        "WredDrp/pkts": "27",
        "WredDrp/bytes": "19",
        "EcnMarked/pkts": "49",
        "EcnMarked/bytes": "80"
    },
    "Ethernet4:8": {
        "WredDrp/pkts": "13",
        "WredDrp/bytes": "89",
        "EcnMarked/pkts": "13",
        "EcnMarked/bytes": "33"
    },
    "Ethernet4:9": {
        "WredDrp/pkts": "43",
        "WredDrp/bytes": "48",
        "EcnMarked/pkts": "86",
        "EcnMarked/bytes": "31"
    },
    "Ethernet4:10": {
        "WredDrp/pkts": "50",
        "WredDrp/bytes": "1",
        "EcnMarked/pkts": "57",
        "EcnMarked/bytes": "82"
    },
    "Ethernet4:11": {
        "WredDrp/pkts": "67",
        "WredDrp/bytes": "99",
        "EcnMarked/pkts": "84",
        "EcnMarked/bytes": "59"
    },
    "Ethernet4:12": {
        "WredDrp/pkts": "4",
        "WredDrp/bytes": "58",
        "EcnMarked/pkts": "27",
        "EcnMarked/bytes": "5"
    },
    "Ethernet4:13": {
        "WredDrp/pkts": "74",
        "WredDrp/bytes": "5",
        "EcnMarked/pkts": "57",
        "EcnMarked/bytes": "39"
    },
    "Ethernet4:14": {
        "WredDrp/pkts": "21",
        "WredDrp/bytes": "59",
        "EcnMarked/pkts": "4",
        "EcnMarked/bytes": "14"
    },
    "Ethernet4:15": {
        "WredDrp/pkts": "24",
        "WredDrp/bytes": "61",
        "EcnMarked/pkts": "19",
        "EcnMarked/bytes": "53"
    },
    "Ethernet4:16": {
        "WredDrp/pkts": "51",
        "WredDrp/bytes": "15",
        "EcnMarked/pkts": "15",
        "EcnMarked/bytes": "32"
    },
    "Ethernet4:17": {
        "WredDrp/pkts": "98",
        "WredDrp/bytes": "18",
        "EcnMarked/pkts": "23",
        "EcnMarked/bytes": "15"
    },
    "Ethernet4:18": {
        "WredDrp/pkts": "41",
        "WredDrp/bytes": "34",
        "EcnMarked/pkts": "9",
        "EcnMarked/bytes": "57"
    },
    "Ethernet4:19": {
        "WredDrp/pkts": "57",
        "WredDrp/bytes": "7",
        "EcnMarked/pkts": "18",
        "EcnMarked/bytes": "99"
    },
    "Ethernet4:20": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet4:21": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet4:22": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet4:23": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet4:24": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet4:25": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet4:26": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet4:27": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet4:28": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet4:29": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet8:0": {
        "WredDrp/pkts": "0",
        "WredDrp/bytes": "0",
        "EcnMarked/pkts": "0",
        "EcnMarked/bytes": "0"
    },
    "Ethernet8:1": {
        "WredDrp/pkts": "38",
        "WredDrp/bytes": "17",
        "EcnMarked/pkts": "68",
        "EcnMarked/bytes": "91"
    },
    "Ethernet8:2": {
        "WredDrp/pkts": "16",
        "WredDrp/bytes": "65",
        "EcnMarked/pkts": "79",
        "EcnMarked/bytes": "51"
    },
    "Ethernet8:3": {
        "WredDrp/pkts": "11",
        "WredDrp/bytes": "97",
        "EcnMarked/pkts": "63",
        "EcnMarked/bytes": "72"
    },
    "Ethernet8:4": {
        "WredDrp/pkts": "54",
        "WredDrp/bytes": "89",
        "EcnMarked/pkts": "62",
        "EcnMarked/bytes": "62"
    },
    "Ethernet8:5": {
        "WredDrp/pkts": "13",
        "WredDrp/bytes": "84",
        "EcnMarked/pkts": "30",
        "EcnMarked/bytes": "59"
    },
    "Ethernet8:6": {
        "WredDrp/pkts": "49",
        "WredDrp/bytes": "67",
        "EcnMarked/pkts": "99",
        "EcnMarked/bytes": "85"
    },
    "Ethernet8:7": {
        "WredDrp/pkts": "2",
        "WredDrp/bytes": "63",
        "EcnMarked/pkts": "38",
        "EcnMarked/bytes": "88"
    },
    "Ethernet8:8": {
        "WredDrp/pkts": "0",
        "WredDrp/bytes": "82",
        "EcnMarked/pkts": "93",
        "EcnMarked/bytes": "43"
    },
    "Ethernet8:9": {
        "WredDrp/pkts": "80",
        "WredDrp/bytes": "17",
        "EcnMarked/pkts": "91",
        "EcnMarked/bytes": "61"
    },
    "Ethernet8:10": {
        "WredDrp/pkts": "81",
        "WredDrp/bytes": "63",
        "EcnMarked/pkts": "76",
        "EcnMarked/bytes": "73"
    },
    "Ethernet8:11": {
        "WredDrp/pkts": "29",
        "WredDrp/bytes": "16",
        "EcnMarked/pkts": "29",
        "EcnMarked/bytes": "66"
    },
    "Ethernet8:12": {
        "WredDrp/pkts": "32",
        "WredDrp/bytes": "12",
        "EcnMarked/pkts": "61",
        "EcnMarked/bytes": "35"
    },
    "Ethernet8:13": {
        "WredDrp/pkts": "79",
        "WredDrp/bytes": "17",
        "EcnMarked/pkts": "72",
        "EcnMarked/bytes": "93"
    },
    "Ethernet8:14": {
        "WredDrp/pkts": "23",
        "WredDrp/bytes": "21",
        "EcnMarked/pkts": "67",
        "EcnMarked/bytes": "50"
    },
    "Ethernet8:15": {
        "WredDrp/pkts": "37",
        "WredDrp/bytes": "10",
        "EcnMarked/pkts": "97",
        "EcnMarked/bytes": "14"
    },
    "Ethernet8:16": {
        "WredDrp/pkts": "30",
        "WredDrp/bytes": "17",
        "EcnMarked/pkts": "74",
        "EcnMarked/bytes": "43"
    },
    "Ethernet8:17": {
        "WredDrp/pkts": "0",
        "WredDrp/bytes": "63",
        "EcnMarked/pkts": "54",
        "EcnMarked/bytes": "84"
    },
    "Ethernet8:18": {
        "WredDrp/pkts": "69",
        "WredDrp/bytes": "88",
        "EcnMarked/pkts": "24",
        "EcnMarked/bytes": "79"
    },
    "Ethernet8:19": {
        "WredDrp/pkts": "20",
        "WredDrp/bytes": "12",
        "EcnMarked/pkts": "84",
        "EcnMarked/bytes": "3"
    },
    "Ethernet8:20": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet8:21": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet8:22": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet8:23": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet8:24": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet8:25": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet8:26": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet8:27": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet8:28": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    },
    "Ethernet8:29": {
        "WredDrp/pkts": "N/A",
        "WredDrp/bytes": "N/A",
        "EcnMarked/pkts": "N/A",
        "EcnMarked/bytes": "N/A"
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
     Port    TxQ    WredDrp/pkts    WredDrp/bytes    EcnMarked/pkts    EcnMarked/bytes
---------  -----  --------------  ---------------  ----------------  -----------------
Ethernet0    UC0               0                0                 0                  0
Ethernet0    UC1              60               43                39                  1
Ethernet0    UC2              82                7                39                 21
Ethernet0    UC3              52               70                19                 76
Ethernet0    UC4              11               59                12                 94
Ethernet0    UC5              36               62                35                 40
Ethernet0    UC6              49               91                 2                 88
Ethernet0    UC7              33               17                94                 74
Ethernet0    UC8              40               71                95                 33
Ethernet0    UC9              54                8                93                 78
Ethernet0   MC10              83               96                74                  9
Ethernet0   MC11              15               60                61                 31
Ethernet0   MC12              45               52                82                 94
Ethernet0   MC13              55               88                89                 52
Ethernet0   MC14              14               70                95                 79
Ethernet0   MC15              68               60                66                 81
Ethernet0   MC16              63                4                48                 76
Ethernet0   MC17              41               73                77                 74
Ethernet0   MC18              60               21                56                 54
Ethernet0   MC19              57               31                12                 39
Ethernet0  ALL20             N/A              N/A               N/A                N/A
Ethernet0  ALL21             N/A              N/A               N/A                N/A
Ethernet0  ALL22             N/A              N/A               N/A                N/A
Ethernet0  ALL23             N/A              N/A               N/A                N/A
Ethernet0  ALL24             N/A              N/A               N/A                N/A
Ethernet0  ALL25             N/A              N/A               N/A                N/A
Ethernet0  ALL26             N/A              N/A               N/A                N/A
Ethernet0  ALL27             N/A              N/A               N/A                N/A
Ethernet0  ALL28             N/A              N/A               N/A                N/A
Ethernet0  ALL29             N/A              N/A               N/A                N/A

     Port    TxQ    WredDrp/pkts    WredDrp/bytes    EcnMarked/pkts    EcnMarked/bytes
---------  -----  --------------  ---------------  ----------------  -----------------
Ethernet4    UC0              41               96                70                 98
Ethernet4    UC1              18               49                63                 36
Ethernet4    UC2              99               90                 3                 15
Ethernet4    UC3              60               89                48                 41
Ethernet4    UC4               8               84                82                 94
Ethernet4    UC5              83               15                75                 92
Ethernet4    UC6              84               26                50                 71
Ethernet4    UC7              27               19                49                 80
Ethernet4    UC8              13               89                13                 33
Ethernet4    UC9              43               48                86                 31
Ethernet4   MC10              50                1                57                 82
Ethernet4   MC11              67               99                84                 59
Ethernet4   MC12               4               58                27                  5
Ethernet4   MC13              74                5                57                 39
Ethernet4   MC14              21               59                 4                 14
Ethernet4   MC15              24               61                19                 53
Ethernet4   MC16              51               15                15                 32
Ethernet4   MC17              98               18                23                 15
Ethernet4   MC18              41               34                 9                 57
Ethernet4   MC19              57                7                18                 99
Ethernet4  ALL20             N/A              N/A               N/A                N/A
Ethernet4  ALL21             N/A              N/A               N/A                N/A
Ethernet4  ALL22             N/A              N/A               N/A                N/A
Ethernet4  ALL23             N/A              N/A               N/A                N/A
Ethernet4  ALL24             N/A              N/A               N/A                N/A
Ethernet4  ALL25             N/A              N/A               N/A                N/A
Ethernet4  ALL26             N/A              N/A               N/A                N/A
Ethernet4  ALL27             N/A              N/A               N/A                N/A
Ethernet4  ALL28             N/A              N/A               N/A                N/A
Ethernet4  ALL29             N/A              N/A               N/A                N/A

     Port    TxQ    WredDrp/pkts    WredDrp/bytes    EcnMarked/pkts    EcnMarked/bytes
---------  -----  --------------  ---------------  ----------------  -----------------
Ethernet8    UC0               0                0                 0                  0
Ethernet8    UC1              38               17                68                 91
Ethernet8    UC2              16               65                79                 51
Ethernet8    UC3              11               97                63                 72
Ethernet8    UC4              54               89                62                 62
Ethernet8    UC5              13               84                30                 59
Ethernet8    UC6              49               67                99                 85
Ethernet8    UC7               2               63                38                 88
Ethernet8    UC8               0               82                93                 43
Ethernet8    UC9              80               17                91                 61
Ethernet8   MC10              81               63                76                 73
Ethernet8   MC11              29               16                29                 66
Ethernet8   MC12              32               12                61                 35
Ethernet8   MC13              79               17                72                 93
Ethernet8   MC14              23               21                67                 50
Ethernet8   MC15              37               10                97                 14
Ethernet8   MC16              30               17                74                 43
Ethernet8   MC17               0               63                54                 84
Ethernet8   MC18              69               88                24                 79
Ethernet8   MC19              20               12                84                  3
Ethernet8  ALL20             N/A              N/A               N/A                N/A
Ethernet8  ALL21             N/A              N/A               N/A                N/A
Ethernet8  ALL22             N/A              N/A               N/A                N/A
Ethernet8  ALL23             N/A              N/A               N/A                N/A
Ethernet8  ALL24             N/A              N/A               N/A                N/A
Ethernet8  ALL25             N/A              N/A               N/A                N/A
Ethernet8  ALL26             N/A              N/A               N/A                N/A
Ethernet8  ALL27             N/A              N/A               N/A                N/A
Ethernet8  ALL28             N/A              N/A               N/A                N/A
Ethernet8  ALL29             N/A              N/A               N/A                N/A"""


@pytest.mark.sonic_baseline
class TestQueueWredcounters:
    """
    Baseline test for 'show queue wredcounters' (Cmd #62).

    Mirrors sonic-utilities test:
      File: wred_queue_counter_test.py
      Func: test_show_wred_queue_counters
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
