impl Solution {
    // pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
    //
    // }
}

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
