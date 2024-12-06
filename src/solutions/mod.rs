macro_rules! export_solutions {
    ($($module:ident),* $(,)?) => (
        $(mod $module;)*

        pub fn solutions() -> &'static [(&'static str, fn(Vec<&str>) -> String)] {
            &[
                $((stringify!($module), $module::solution),)*
            ]
        }
    );
}

export_solutions!(
    day1_p1, day1_p2, day2_p1, day2_p2, day3_p1, day3_p2, day4_p1, day4_p2, day5_p1, day5_p2,
    day6_p1, day6_p2
);
