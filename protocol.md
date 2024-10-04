# Protocol notes

First analyses, tshark:
```
tshark -r 2024_09_21_2343_capture_fan_and_pump_profile.pcapng -Y "(usb.dst == \"1.3.0\") or (usb.src==\"1.3.0\")" -T fields -e frame.time -e usb.src -e usb.dst -e usb.data_fragment

usb.dst == "1.3.0" or usb.src == "1.3.0"
```


Captured on windows with deliberate actions:
```
preamble; 1c00204adf0482daffff000000001b000001000300000248000000002109000200004000
                                                            | random data, forgot init?                                      | checksum?
3f101400ff05ffffffffff00000000000000000000000000ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42affffffffffff59
pump quiet
3f101400ff05ffffffffff00000000000000000000000000ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42affffffffffff59
pump balanced
3fd81400ff05ffffffffff00000000000000000000000001ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42affffffffffff4d
pump extreme
3f581400ff05ffffffffff00000000000000000000000002ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42affffffffffff43
coolant at 35.70 ish

fan1 balanced
3f581400ff05ffffffffff00000000000000000000000002ffff0000ff071c331e4f1f69208722ad23d424ff1e33204f2169238725ad29d42affffffffffff3a

fan1 extreme
3fa81400ff05ffffffffff00000000000000000000000002ffff0000ff071a591b6e1c871da31ebd1fdb20ff1e33204f2169238725ad29d42afffffffffffff2

fan1 quiet
3fc81400ff05ffffffffff00000000000000000000000002ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42affffffffffff40

fan2 balanced
3f301400ff05ffffffffff00000000000000000000000002ffff0000ff071e33204f2169238725ad29d42aff1c331e4f1f69208722ad23d424fffffffffffff8

fan2 extreme
3fa01400ff05ffffffffff00000000000000000000000002ffff0000ff071e33204f2169238725ad29d42aff1a591b6e1c871da31ebd1fdb20ffffffffffffcf

fan2 quiet
3f081400ff05ffffffffff00000000000000000000000002ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42afffffffffffff7
```

From `ServiceLogs\Corsair_Cooling`:
```
2024-09-28 20:32:54.7619 | 42 | INFO | H100i ELITE | ============================================================ 
2024-09-28 20:32:54.7619 | 42 | INFO | H100i ELITE | Write performance settings (Fan #0, Mode = DefaultUsingInternalTemp) 
2024-09-28 20:32:54.7619 | 42 | INFO | H100i ELITE | ============================================================ 
2024-09-28 20:32:54.7619 | 42 | INFO | H100i ELITE | Temp:	30	32	33	35	37	41	42 
2024-09-28 20:32:54.7619 | 42 | INFO | H100i ELITE | PWM:	20	31	41	53	68	83	100 
2024-09-28 20:32:54.7619 | 42 | INFO | H100i ELITE | ============================================================ 
2024-09-28 20:32:54.7619 | 42 | INFO | H100i ELITE | ============================================================ 
2024-09-28 20:32:54.7619 | 42 | INFO | H100i ELITE | Write performance settings (Fan #1, Mode = DefaultUsingInternalTemp) 
2024-09-28 20:32:54.7619 | 42 | INFO | H100i ELITE | ============================================================ 
2024-09-28 20:32:54.7619 | 42 | INFO | H100i ELITE | Temp:	30	32	33	35	37	41	42 
2024-09-28 20:32:54.7619 | 42 | INFO | H100i ELITE | PWM:	20	31	41	53	68	83	100 
2024-09-28 20:32:54.7619 | 42 | INFO | H100i ELITE | ============================================================ 
2024-09-28 20:32:54.7619 | 42 | INFO | H100i ELITE | New default curves were stored to device 
2024-09-28 20:32:54.7619 | 42 | INFO | H100i ELITE | Pump mode 'Balanced' was stored to device 
```

pwm conversions:
```
z = lambda d: print(int((d/100)*255.0), hex(int((d/100)*255.0)))
z(20) = 51 0x33
z(31) = 79 0x4f
z(41) = 104 0x68
z(53) = 135 0x87
z(68) = 173 0xad
z(83) = 211 0xd3
z(100) = 255 0xff

                                                                                          __  __  ??  __  __  ??  __
3fd81400ff05ffffffffff00000000000000000000000001ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42affffffffffff4d

69 and d4 don't match...
z = lambda d: print(math.ceil((d/100)*255.0), hex(math.ceil((d/100)*255.0)))
z(41) = 105 0x69
z(83) = 212 0xd4

bingo.

PWMs                                           P            |                           | __  __  __  __  __  __  __
3fd81400ff05ffffffffff00000000000000000000000001ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42affffffffffff4d
Temp                                                                                    30  32  33  35  37  41  42

P is pump.
Quiet is 0
balanced is 1
extreme is 2

Another 14 thing is this: 
3f681400ff05ffffffffffffffffffffffffffffffffff02ffffd422ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff36
                                                    ^^ ~only variation~
                                                    ^^^^ only variation

tshark -r 2024_09_28*pcapng -Y "(usb.dst == \"1.6.0\") or (usb.src==\"1.6.0\") or (usb.dst==\"1.6.1\") or (usb.src==\"1.6.1\")" -T fields -e usb.src -e usb.dst -e usb.data_fragment -e usb.capdata|  rg ':02:ff:ff:\d\d:\d\d:'

3f781400ff05ffffffffffffffffffffffffffffffffff02ffff8622ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff17
3f801400ff05ffffffffffffffffffffffffffffffffff02ffff8922ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe3
3f981400ff05ffffffffffffffffffffffffffffffffff02ffff9122ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff79
3fa01400ff05ffffffffffffffffffffffffffffffffff02ffff9422ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff47
3fa81400ff05ffffffffffffffffffffffffffffffffff02ffff9822fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe
3fa01400ff05ffffffffffffffffffffffffffffffffff02ffff0023ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffb7
3fa81400ff05ffffffffffffffffffffffffffffffffff02ffff0023ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff58
3fb01400ff05ffffffffffffffffffffffffffffffffff02ffff0123ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1
3fb81400ff05ffffffffffffffffffffffffffffffffff02ffff0223ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff98
3fc01400ff05ffffffffffffffffffffffffffffffffff02ffff0223ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff76
3fc81400ff05ffffffffffffffffffffffffffffffffff02ffff0323ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff16
3fd01400ff05ffffffffffffffffffffffffffffffffff02ffff0323ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff20
3fd81400ff05ffffffffffffffffffffffffffffffffff02ffff0423ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6b
3fe01400ff05ffffffffffffffffffffffffffffffffff02ffff0523ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff67
3ff01400ff05ffffffffffffffffffffffffffffffffff02ffff0523ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffbe
3ff81400ff05ffffffffffffffffffffffffffffffffff02ffff0623ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc7
3f081400ff05ffffffffffffffffffffffffffffffffff02ffff0623ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff1c

```
Does look like a TempStatus being written? `2` combined with `0000` in this field sets the pump to extreme... perhaps it sets the pump to go from 75% to 100% at this temperature?





Pump settings & fan settings start with `0x3fxx14`

Requests come back from another endpoint?
```
tshark -r 2024_09_21_2343_capture_fan_and_pump_profile.pcapng -Y "(usb.dst == \"1.3.1\") or (usb.src==\"1.3.1\")" -T fields -e frame.time -e usb.src -e usb.dst -e usb.capdata
ff:a0:12:08:00:17:13:92:24:00:00:a5:e8:03:a5:de:04:00:a5:e8:03:a5:ee:04:02:ff:00:00:ff:7f:0b:00:00:00:00:00:00:00:00:45:32:33:45:00:00:81:8b:66:00:05:2d:32:37:41:80:24:00:00:00:00:00:00:00:44
ff:a8:12:08:00:18:13:92:24:00:00:a5:e8:03:a5:de:04:00:a5:e8:03:a5:ee:04:02:ff:00:00:ff:7f:0b:00:00:00:00:00:00:00:00:45:32:33:45:00:00:a7:8b:66:00:05:2d:32:37:41:80:24:00:00:00:00:00:00:00:b2

tshark -r 2024_09_21_2343_capture_fan_and_pump_profile.pcapng -Y "(usb.dst == \"1.3.0\") or (usb.src==\"1.3.0\") or (usb.dst == \"1.3.1\") or (usb.src == \"1.3.1\") " -T fields -e frame.time -e usb.src -e usb.dst -e usb.data_fragment -e usb.capdata
```

Requests and responses in one:
```
tshark -r 2024_09_21_2343_capture_fan_and_pump_profile.pcapng -Y "(usb.dst == \"1.3.0\") or (usb.src==\"1.3.0\") or (usb.dst == \"1.3.1\") or (usb.src == \"1.3.1\") " -T fields -e frame.time -e usb.src -e usb.dst -e usb.data_fragment
```

Requesting data (fan speed, temperature)?

```
Sep 21, 2024 19:42:56.154897000 EDT	host	1.3.0	3f:c0:ff:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:35	
Sep 21, 2024 19:42:56.155178000 EDT	1.3.0	host		
Sep 21, 2024 19:42:56.159726000 EDT	1.3.1	host		ff:88:12:08:00:9f:15:7d:23:00:00:90:e8:03:90:40:04:00:90:e8:03:90:52:04:02:ff:00:00:ff:8b:0b:00:00:00:00:00:00:00:00:45:32:33:45:00:00:40:e3:6f:00:05:2d:32:37:41:6a:23:00:00:00:00:00:00:00:d3
Sep 21, 2024 19:42:56.159755000 EDT	host	1.3.1		
Sep 21, 2024 19:42:57.669453000 EDT	host	1.3.0	3f:c8:ff:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:da	
Sep 21, 2024 19:42:57.669702000 EDT	1.3.0	host		
Sep 21, 2024 19:42:57.675657000 EDT	1.3.1	host		ff:90:12:08:00:a0:15:7a:23:00:00:90:e8:03:90:41:04:00:90:e8:03:90:52:04:02:ff:00:00:ff:81:0b:00:00:00:00:00:00:00:00:45:32:33:45:00:00:36:e9:6f:00:05:2d:32:37:41:6a:23:00:00:00:00:00:00:00:d4
Sep 21, 2024 19:42:57.675664000 EDT	host	1.3.1		
Sep 21, 2024 19:42:59.199969000 EDT	host	1.3.0	3f:d0:ff:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:ec	
Sep 21, 2024 19:42:59.200252000 EDT	1.3.0	host		
Sep 21, 2024 19:42:59.203696000 EDT	1.3.1	host		ff:98:12:08:00:a1:15:77:23:00:00:90:e8:03:90:41:04:00:90:e8:03:90:52:04:02:ff:00:00:ff:79:0b:00:00:00:00:00:00:00:00:45:32:33:45:00:00:3c:ef:6f:00:05:2d:32:37:41:6a:23:00:00:00:00:00:00:00:81
```

Status maybe `0x3fxxff`

second byte is sequence number, but it always increments by 8?

Reply is in URB_INTERRUPT

New capture, on icue start:
```
tshark -r 2024_09_28*pcapng -Y "(usb.dst == \"1.6.0\") or (usb.src==\"1.6.0\") or (usb.dst==\"1.6.1\") or (usb.src==\"1.6.1\")" -T fields -e frame.time -e usb.src -e usb.dst -e usb.data_fragment -e usb.capdata
```

## Checksum

```
./reveng -s -w 8 c0ff00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000035    c8ff000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000da  b8ff000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000db 40ff000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ef 
width=8  poly=0x07  init=0x00  refin=false  refout=false  xorout=0x00  check=0xf4  residue=0x00  name="CRC-8"
```

On outgoing requests, first byte is missing from crc calculation.
Same on incoming data;
```
./reveng -s -w 8 881208009f157d23000090e8039040040090e80390520402ff0000ff8b0b000000000000000045323345000040e36f00052d3237416a2300000000000000d3 90120800a0157a23000090e8039041040090e80390520402ff0000ff810b000000000000000045323345000036e96f00052d3237416a2300000000000000d4
width=8  poly=0x07  init=0x00  refin=false  refout=false  xorout=0x00  check=0xf4  residue=0x00  name="CRC-8"
```

## ImHex pattern struct

For a consecutive file of status messages, each being 64 bytes long, concatenated together; `2024_09_26_100ms_status_reg_retrieval.txt`

Switching to comments in the rust code for status.
```
#pragma pattern_limit 20000000

// whats t1 and t2, definitely correlate

struct RepeatPattern{
    u8 command_duty_perhaps;
    u8 e8; // only e8 for fan, 0 for pump?
    u8 is03; // only 03 for fan, 0 for pump?
    u8 control_duty_perhaps; // both dutys are identical?
    u16 value; // does this need a divisor for fans?
//    u8 pair_val2;
    
    u8 padthing;
} [[static]];

struct Values {
    u8 always_12;
    u8 always_08;
    u8 pad1[1];
    u16 msg_counter;
    u16 first_value_t1;
    u16 pad2; // always zeros

    RepeatPattern v1;
    RepeatPattern v2;
    RepeatPattern v3;
    RepeatPattern v4;
    
    u16 something_le;  // is this a temperature in Kelvin?
    be u16 something_be;
    u8 buf[2];
    u32 uptime_ms; // pretty sure about this, increments change exactly with used delay.
    u8 some_id[5]; //0x052d323741
    u16 last_value_t2;
    
    u8 buf_zero[7];
    u8 crc;
} [[static]];

struct Msg {
    u8 cmd;
    u8 seq;
    if (cmd == 0xff) {
      Values v;
    } else {
     u8 zzz[62];
    }
};

Msg v[4500] @ 0;
```

## 09-29 session

Service startup & icue startup
```
tshark -r 2024_09_29_2234* -Y "(usb.dst == \"1.3.0\") or (usb.src==\"1.3.0\") or (usb.dst==\"1.3.1\") or (usb.src==\"1.3.1\")" -T fields  -e usb.src -e usb.dst -e usb.data_fragment -e usb.capdata
```

Not that much interesting unfortunately.

Volatile pump changes:
```
tshark -r 2024_09_29_2236_p* -Y "(usb.dst == \"1.3.0\") or (usb.src==\"1.3.0\") or (usb.dst==\"1.3.1\") or (usb.src==\"1.3.1\")" -T fields  -e usb.src -e usb.dst -e usb.data_fragment -e usb.capdata
Quiet
3f:98:14:00:ff:05:ff:ff:ff:ff:ff:00:00:00:00:00:00:00:00:00:00:00:00:00:ff:ff:00:00:ff:07:1e:33:20:4f:21:69:23:87:25:ad:29:d4:2a:ff:1e:33:20:4f:21:69:23:87:25:ad:29:d4:2a:ff:ff:ff:ff:ff:ff:6c
balanced
3f:d8:14:00:ff:05:ff:ff:ff:ff:ff:00:00:00:00:00:00:00:00:00:00:00:00:01:ff:ff:00:00:ff:07:1e:33:20:4f:21:69:23:87:25:ad:29:d4:2a:ff:1e:33:20:4f:21:69:23:87:25:ad:29:d4:2a:ff:ff:ff:ff:ff:ff:4d
Extreme
3f:10:14:00:ff:05:ff:ff:ff:ff:ff:00:00:00:00:00:00:00:00:00:00:00:00:02:ff:ff:00:00:ff:07:1e:33:20:4f:21:69:23:87:25:ad:29:d4:2a:ff:1e:33:20:4f:21:69:23:87:25:ad:29:d4:2a:ff:ff:ff:ff:ff:ff:c1
Variable Speed
3f:48:14:00:ff:05:ff:ff:ff:ff:ff:00:00:00:00:00:00:00:00:00:00:00:00:00:ff:ff:00:00:ff:07:1e:33:20:4f:21:69:23:87:25:ad:29:d4:2a:ff:1e:33:20:4f:21:69:23:87:25:ad:29:d4:2a:ff:ff:ff:ff:ff:ff:02
```
It appears variable speed is secretly just the Quiet pump profile, but then the service probably overrides the fan speed?


Device cooling flash `device_cooling_flash_pump_balanced_fans_quiet`
```
3f:60:14:00:ff:05:ff:ff:ff:ff:ff:00:00:00:00:00:00:00:00:00:00:00:00:02:ff:ff:00:00:ff:07:1e:33:20:4f:21:69:23:87:25:ad:29:d4:2a:ff:1e:33:20:4f:21:69:23:87:25:ad:29:d4:2a:ff:ff:ff:ff:ff:ff:c0
3f:d0:14:00:ff:05:ff:ff:ff:ff:ff:00:00:00:00:00:00:00:00:00:00:00:00:01:ff:ff:00:00:ff:07:1e:33:20:4f:21:69:23:87:25:ad:29:d4:2a:ff:1e:33:20:4f:21:69:23:87:25:ad:29:d4:2a:ff:ff:ff:ff:ff:ff:a2
3f:e8:14:00:ff:05:ff:ff:ff:ff:ff:00:00:00:00:00:00:00:00:00:00:00:00:01:ff:ff:00:00:ff:07:1e:33:20:4f:21:69:23:87:25:ad:29:d4:2a:ff:1e:33:20:4f:21:69:23:87:25:ad:29:d4:2a:ff:ff:ff:ff:ff:ff:21
```
No difference from the pump changes above!?

Did the capture buffer flush?




