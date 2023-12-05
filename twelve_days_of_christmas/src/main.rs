fn main() {
    for number in 1..13 {
        on_the_nth_day(number);

        if number >= 12 {
            twelve_drummers_drumming();
        }

        if number >= 11 {
            eleven_pipers_piping();
        }

        if number >= 10 {
            ten_lords_aleaping();
        }

        if number >= 9 {
            nine_ladies_dancing();
        }

        if number >= 8 {
            eight_maids_amilking();
        }

        if number >= 7 {
            seven_swans_aswanning();
        }

        if number >= 6 {
            six_geese_alaying();
        }

        if number >= 5 {
            five_gold_riiiiiings();
        }

        if number >= 4 {
            four_calling_birds();
        }

        if number >= 3 {
            three_french_hens();
        }

        if number >= 2 {
            two_turtle_doves();
        }

        if number > 1 {
            and_a_partridge();
        }

        if number == 1 {
            println!("A partridge in a pear tree.\n");
        }
    }
}

fn on_the_nth_day(n: u32) {
    let ordinal;

    if n == 1 {
        ordinal = "st";
    } else if n == 2 {
        ordinal = "nd";
    } else if n == 3 {
        ordinal = "rd";
    } else {
        ordinal = "th";
    }

    println!("On the {n}{ordinal} day of Christmas,\nMy true love sent to me");
}

fn and_a_partridge() {
    println!("And a partridge in a pear tree.\n");
}

fn two_turtle_doves() {
    println!("Two turtle doves,");
}

fn three_french_hens() {
    println!("Three French hens,");
}

fn four_calling_birds() {
    println!("Four calling birds,");
}

fn five_gold_riiiiiings() {
    println!("Five gold riiiiiings,");
}

fn six_geese_alaying() {
    println!("Six geese a-laying,");
}

fn seven_swans_aswanning() {
    println!("Seven swans a-swimming,");
}

fn eight_maids_amilking() {
    println!("Eight maids a-milking,");
}

fn nine_ladies_dancing() {
    println!("Nine ladies dancing,");
}

fn ten_lords_aleaping() {
    println!("Ten lords a-leaping,");
}

fn eleven_pipers_piping() {
    println!("Eleven pipers piping,");
}

fn twelve_drummers_drumming() {
    println!("Twelve drummers drumming,");
}
