var searchIndex = {};
searchIndex['calendar_queue'] = {"items":[[0,"","calendar_queue","This crate implements the idea of a \"Calendar Queue Scheduler\" data structure.",null,null],[3,"CalendarQueue","","",null,null],[12,"senders","","",0,null],[12,"conformance_times","","",0,null],[4,"Error","","",null,null],[13,"DuplicateFlowId","","",1,null],[13,"FlowDoesntExist","","",1,null],[11,"new","","```rust\nuse calendar_queue::CalendarQueue;\nlet _ = CalendarQueue::<u64, String>::new();\n```",0,{"inputs":[{"name":"calendarqueue"}],"output":{"name":"self"}}],[11,"create_channel","","Creates a `mpsc::Sender` with the given ID that hooks into the queue.",0,{"inputs":[{"name":"calendarqueue"},{"name":"i"},{"name":"conformanceticks"}],"output":{"name":"result"}}],[11,"add_channel","","Hooks a `mpsc::Sender` with the given ID into the queue.",0,{"inputs":[{"name":"calendarqueue"},{"name":"receiver"},{"name":"i"},{"name":"conformanceticks"}],"output":{"name":"result"}}],[11,"tick","","```rust\nuse calendar_queue::CalendarQueue;\nlet mut queue = CalendarQueue::<u64, String>::new();\nlet (sender, receiver) = std::sync::mpsc::channel();\nqueue.add_channel(receiver, 1, 3)\n    .unwrap();\nsender.send(\"Foo\".into())\n    .unwrap();\nsender.send(\"Bar\".into())\n    .unwrap();\nassert_eq!(queue.tick(), Some(\"Foo\".into()));\nassert_eq!(queue.tick(), None);\nassert_eq!(queue.tick(), None);\nassert_eq!(queue.tick(), Some(\"Bar\".into()));\nassert_eq!(queue.tick(), None);\nassert_eq!(queue.tick(), None);\nassert_eq!(queue.tick(), None);\n```",0,{"inputs":[{"name":"calendarqueue"}],"output":{"name":"option"}}],[11,"next","","Ticks until it finds something.",0,{"inputs":[{"name":"calendarqueue"}],"output":{"name":"option"}}],[6,"ConformanceTicks","","",null,null],[6,"ClockTick","","",null,null],[6,"Result","","",null,null],[11,"eq","","",1,{"inputs":[{"name":"error"},{"name":"error"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"error"},{"name":"error"}],"output":{"name":"bool"}}],[11,"fmt","","",1,{"inputs":[{"name":"error"},{"name":"formatter"}],"output":{"name":"result"}}]],"paths":[[3,"CalendarQueue"],[4,"Error"]]};
initSearch(searchIndex);
