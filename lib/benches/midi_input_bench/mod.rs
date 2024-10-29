use criterion::Criterion;
use rust_dsp::{
  midibitfield::MidiBitField,
};

use std::collections::{VecDeque, HashSet};




fn vector_u8(test_data: &[(u8, bool)], vec: &mut Vec<u8>) {
  for (note, insert) in test_data {
    if *insert {
      vec.push(*note);
    } 
    if let Some(i) = vec.iter().position(|x| *x == *note) {
      vec.remove(i);
    }
  }

  for v in vec {
    core::hint::black_box(&v);
  }
}

fn hashset_u8(test_data: &[(u8, bool)], hash: &mut HashSet<u8>) {
  for (note, insert) in test_data {
    if *insert { hash.insert(*note);
    } else { hash.remove(note); }
  }

  for v in hash.iter() {
    core::hint::black_box(&v);
  }
}

fn bitfield_u8(test_data: &[(u8, bool)], bf: &mut MidiBitField) {
  for (note, insert) in test_data {
    if *insert { bf.add(*note).unwrap();
    } else { bf.remove(*note).unwrap(); }
  }

  bf.notes(&mut |note| {
    core::hint::black_box(&note);
  })


}


pub fn criterion_benchmark_midi(c: &mut Criterion) {
  const SIZE: usize = 128;
  const MAX: usize = 32;
  let test_data: [(u8, bool); 40] = [
    (17, false), (12, true), (6, true), (9, false), 
    (20, true), (14, false), (3, true), (11, false), 
    (2, false), (13, true), (8, true), (19, false), 
    (1, false), (5, true), (4, false), (7, false), 
    (15, true), (10, false), (16, false), (18, true), 
    (17, true), (12, false), (6, false), (9, true), 
    (20, false), (14, true), (3, false), (11, true), 
    (2, true), (13, false), (8, false), (19, true), 
    (1, true), (5, false), (4, true), (7, true), 
    (15, false), (10, true), (16, true), (18, false),
];
//   let test_data: [(u8, bool); 705] = [
//     (23, true), (57, false), (112, true), (89, false), (34, true),
//     (76, false), (45, true), (9, false), (120, true), (68, false),
//     (31, true), (101, false), (47, true), (15, false), (88, true),
//     (2, false), (79, true), (93, false), (127, true), (50, false),
//     (12, true), (114, false), (8, true), (100, false), (36, true),
//     (52, false), (91, true), (29, false), (63, true), (41, false),
//     (103, true), (22, false), (77, true), (5, false), (111, true),
//     (86, false), (42, true), (17, false), (90, true), (33, false),
//     (99, true), (54, false), (116, true), (10, false), (67, true),
//     (80, false), (39, true), (1, false), (98, true), (71, false),
//     (25, true), (123, false), (59, true), (6, false), (74, true),
//     (80, false), (39, true), (1, false), (98, true), (71, false),
//     (25, true), (123, false), (59, true), (6, false), (74, true),
//     (44, false), (113, true), (21, false), (95, true), (48, false),
//     (60, true), (87, false), (38, true), (26, false), (108, true),
//     (14, false), (72, true), (51, false), (106, true), (66, false),
//     (16, true), (121, false), (78, true), (3, false), (55, true),
//     (110, false), (46, true), (85, false), (20, true), (104, false),
//     (65, true), (92, false), (30, true), (7, false), (117, true),
//     (49, false), (82, true), (18, false), (97, true), (70, false),
//     (28, true), (119, false), (61, true), (35, false), (105, true),
//     (84, false), (13, true), (56, false), (122, true), (24, false),
//     (81, true), (43, false), (96, true), (75, false), (4, true),
//     (102, false), (53, true), (11, false), (109, true), (64, false),
//     (37, true), (19, false), (125, true), (58, false), (40, true),
//     (80, false), (39, true), (1, false), (98, true), (71, false),
//     (25, true), (123, false), (59, true), (6, false), (74, true),
//     (44, false), (113, true), (21, false), (95, true), (48, false),
//     (60, true), (87, false), (38, true), (26, false), (108, true),
//     (14, false), (72, true), (51, false), (106, true), (66, false),
//     (16, true), (121, false), (78, true), (3, false), (55, true),
//     (110, false), (46, true), (85, false), (20, true), (104, false),
//     (65, true), (92, false), (30, true), (7, false), (117, true),
//     (49, false), (82, true), (18, false), (97, true), (70, false),
//     (28, true), (119, false), (61, true), (35, false), (105, true),
//     (84, false), (13, true), (56, false), (122, true), (24, false),
//     (81, true), (43, false), (96, true), (75, false), (4, true),
//     (102, false), (53, true), (11, false), (109, true), (64, false),
//     (37, true), (19, false), (125, true), (58, false), (40, true),
//     (83, false), (32, true), (118, false), (27, true), (94, false),
//     (69, true), (115, false), (0, true), (62, false), (124, true),
//     (33, false), (48, true), (73, false), (100, true), (45, false),
//     (87, true), (53, false), (69, true), (99, false), (22, true),
//     (56, false), (109, true), (5, false), (44, true), (121, false),
//     (75, true), (1, false), (80, true), (34, false), (107, true),
//     (13, false), (66, true), (18, false), (90, true), (35, false),
//     (102, true), (12, false), (95, true), (8, false), (82, true),
//     (47, false), (41, true), (0, false), (116, true), (67, false),
//     (26, true), (126, false), (93, true), (9, false), (72, true),
//     (14, false), (119, true), (32, false), (55, true), (7, false),
//     (83, false), (32, true), (118, false), (27, true), (94, false),
//     (69, true), (115, false), (0, true), (62, false), (124, true),
//     (33, false), (48, true), (73, false), (100, true), (45, false),
//     (87, true), (53, false), (69, true), (99, false), (22, true),
//     (56, false), (109, true), (5, false), (44, true), (121, false),
//     (75, true), (1, false), (80, true), (34, false), (107, true),
//     (80, false), (39, true), (1, false), (98, true), (71, false),
//     (25, true), (123, false), (59, true), (6, false), (74, true),
//     (44, false), (113, true), (21, false), (95, true), (48, false),
//     (60, true), (87, false), (38, true), (26, false), (108, true),
//     (14, false), (72, true), (51, false), (106, true), (66, false),
//     (16, true), (121, false), (78, true), (3, false), (55, true),
//     (110, false), (46, true), (85, false), (20, true), (104, false),
//     (65, true), (92, false), (30, true), (7, false), (117, true),
//     (49, false), (82, true), (18, false), (97, true), (70, false),
//     (28, true), (119, false), (61, true), (35, false), (105, true),
//     (84, false), (13, true), (56, false), (122, true), (24, false),
//     (81, true), (43, false), (96, true), (75, false), (4, true),
//     (102, false), (53, true), (11, false), (109, true), (64, false),
//     (37, true), (19, false), (125, true), (58, false), (40, true),
//     (83, false), (32, true), (118, false), (27, true), (94, false),
//     (69, true), (115, false), (0, true), (62, false), (124, true),
//     (33, false), (48, true), (73, false), (100, true), (45, false),
//     (87, true), (53, false), (69, true), (99, false), (22, true),
//     (56, false), (109, true), (5, false), (44, true), (121, false),
//     (75, true), (1, false), (80, true), (34, false), (107, true),
//     (13, false), (66, true), (18, false), (90, true), (35, false),
//     (102, true), (12, false), (95, true), (8, false), (82, true),
//     (47, false), (41, true), (0, false), (116, true), (67, false),
//     (26, true), (126, false), (93, true), (9, false), (72, true),
//     (14, false), (119, true), (32, false), (55, true), (7, false),
//     (13, false), (66, true), (18, false), (90, true), (35, false),
//     (102, true), (12, false), (95, true), (8, false), (82, true),
//     (47, false), (41, true), (0, false), (116, true), (67, false),
//     (26, true), (126, false), (93, true), (9, false), (72, true),
//     (80, false), (39, true), (1, false), (98, true), (71, false),
//     (25, true), (123, false), (59, true), (6, false), (74, true),
//     (44, false), (113, true), (21, false), (95, true), (48, false),
//     (60, true), (87, false), (38, true), (26, false), (108, true),
//     (14, false), (72, true), (51, false), (106, true), (66, false),
//     (16, true), (121, false), (78, true), (3, false), (55, true),
//     (110, false), (46, true), (85, false), (20, true), (104, false),
//     (65, true), (92, false), (30, true), (7, false), (117, true),
//     (49, false), (82, true), (18, false), (97, true), (70, false),
//     (28, true), (119, false), (61, true), (35, false), (105, true),
//     (84, false), (13, true), (56, false), (122, true), (24, false),
//     (81, true), (43, false), (96, true), (75, false), (4, true),
//     (102, false), (53, true), (11, false), (109, true), (64, false),
//     (37, true), (19, false), (125, true), (58, false), (40, true),
//     (83, false), (32, true), (118, false), (27, true), (94, false),
//     (69, true), (115, false), (0, true), (62, false), (124, true),
//     (33, false), (48, true), (73, false), (100, true), (45, false),
//     (87, true), (53, false), (69, true), (99, false), (22, true),
//     (56, false), (109, true), (5, false), (44, true), (121, false),
//     (75, true), (1, false), (80, true), (34, false), (107, true),
//     (13, false), (66, true), (18, false), (90, true), (35, false),
//     (102, true), (12, false), (95, true), (8, false), (82, true),
//     (47, false), (41, true), (0, false), (116, true), (67, false),
//     (26, true), (126, false), (93, true), (9, false), (72, true),
//     (14, false), (119, true), (32, false), (55, true), (7, false),
//     (14, false), (119, true), (32, false), (55, true), (7, false),
//     (44, false), (113, true), (21, false), (95, true), (48, false),
//     (60, true), (87, false), (38, true), (26, false), (108, true),
//     (14, false), (72, true), (51, false), (106, true), (66, false),
//     (16, true), (121, false), (78, true), (3, false), (55, true),
//     (110, false), (46, true), (85, false), (20, true), (104, false),
//     (65, true), (92, false), (30, true), (7, false), (117, true),
//     (49, false), (82, true), (18, false), (97, true), (70, false),
//     (28, true), (119, false), (61, true), (35, false), (105, true),
//     (84, false), (13, true), (56, false), (122, true), (24, false),
//     (81, true), (43, false), (96, true), (75, false), (4, true),
//     (102, false), (53, true), (11, false), (109, true), (64, false),
//     (37, true), (19, false), (125, true), (58, false), (40, true),
//     (83, false), (32, true), (118, false), (27, true), (94, false),
//     (69, true), (115, false), (0, true), (62, false), (124, true),
//     (33, false), (48, true), (73, false), (100, true), (45, false),
//     (87, true), (53, false), (69, true), (99, false), (22, true),
//     (56, false), (109, true), (5, false), (44, true), (121, false),
//     (75, true), (1, false), (80, true), (34, false), (107, true),
//     (13, false), (66, true), (18, false), (90, true), (35, false),
//     (102, true), (12, false), (95, true), (8, false), (82, true),
//     (47, false), (41, true), (0, false), (116, true), (67, false),
//     (26, true), (126, false), (93, true), (9, false), (72, true),
//     (14, false), (119, true), (32, false), (55, true), (7, false),
//     (64, true), (20, false), (97, true), (36, false), (110, true),
//     (39, false), (49, true), (15, false), (81, true), (28, false),
//     (123, true), (25, false), (92, true), (68, false), (42, true),
//     (58, false), (84, true), (43, false), (96, true), (70, false),
//     (30, true), (88, false), (17, true), (63, false), (4, true),
//     (74, false), (111, true), (52, false), (94, true), (6, false),
//     (85, true), (57, false), (19, true), (27, false), (61, true),
// ];

  let mut group = c.benchmark_group("midi_note_handling");

  let mut v = vec![0;SIZE];

  let mut h = HashSet::with_capacity(SIZE);

  let mut m = MidiBitField::new();

  group.bench_function("vector",
    |b| 
    b.iter(|| {
      vector_u8(&test_data, &mut v);
    }
  ));

  group.bench_function("hashset", 
    |b| b.iter(|| {
      hashset_u8(&test_data, &mut h);
    }
  ));
  
  group.bench_function("bitfield", 
    |b| b.iter(|| {
      bitfield_u8(&test_data, &mut m);
    }
  ));

}


