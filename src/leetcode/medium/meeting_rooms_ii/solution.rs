impl Solution {
    // pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
    //
    // }
}
//
// fn try_book(to_book: TimeInterval, mut meeting_room: MeetingRoom) -> (MeetingRoom, bool) {
//     let mut booked_intervals = &meeting_room.booked_intervals;
//
//     for (idx, booked) in meeting_room.booked_intervals.iter().enumerate() {
//         match idx {
//             0 => {
//                 if to_book.is_before(booked) {
//                     let mut to_return = vec![to_book];
//                     to_return.append(&mut booked_intervals);
//                     //to_return.extend(booked_intervals);
//                     meeting_room.booked_intervals = to_return;
//
//                     return (meeting_room, true);
//                 }
//             },
//             _ if idx == meeting_room.booked_intervals.len() => {
//                 if to_book.is_after(booked) {
//                     meeting_room.booked_intervals.push(to_book);
//
//                     return (meeting_room, true);
//                 }
//             },
//             _ => {
//                 if to_book.is_between(&meeting_room.booked_intervals[idx - 1], booked) {
//                     let mut b = vec![to_book];
//                     let x = &mut meeting_room.booked_intervals[0..idx].to_vec();
//                     let y = &mut meeting_room.booked_intervals[0..idx].to_vec();
//                     x.append(&mut b);
//                     x.append(&mut y);
//                     //booked_intervals.splice(1..1, b.iter().clone());
//                     return (meeting_room, true);
//                 }
//             }
//         }
//
//         // if idx == 3 {
//         //     meeting_room.booked_intervals.push(to_book);
//         //     break;
//         // }
//     }
//
//     (meeting_room, false)
// }

fn create_new_meeting_room_and_book_with(
    to_book: TimeInterval,
    mut meeting_rooms: Vec<MeetingRoom>,
) -> Vec<MeetingRoom> {
    meeting_rooms.push(MeetingRoom {
        booked_intervals: vec![to_book],
    });

    meeting_rooms
}

#[derive(Clone)]
pub struct MeetingRoom {
    pub booked_intervals: Vec<TimeInterval>,
}

#[derive(Clone)]
pub struct TimeInterval {
    pub start: i32,
    pub end: i32,
}

impl TimeInterval {
    fn is_before(&self, to_compare: &TimeInterval) -> bool {
        self.end <= to_compare.start
    }

    fn is_after(&self, to_compare: &TimeInterval) -> bool {
        self.start >= to_compare.end
    }

    fn is_between(&self, first_interval: &TimeInterval, second_interval: &TimeInterval) -> bool {
        (self.is_after(first_interval) && self.is_before(second_interval))
            || (self.is_after(second_interval) && self.is_before(first_interval))
    }
}

pub struct Solution;

#[cfg(test)]
pub mod solution_test {
    use super::*;
    use rstest::rstest;

    #[rstest(
    base_interval,
    to_compare,
    expected,
    case(TimeInterval { start: 0, end: 1 }, TimeInterval { start: 2, end: 4 }, true),
    )]
    fn exist_should_return_false_when_can_find_word_in_board(
        base_interval: TimeInterval,
        to_compare: TimeInterval,
        expected: bool,
    ) {
        assert_eq!(base_interval.is_before(&to_compare), expected);
    }
}
