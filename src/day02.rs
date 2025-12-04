const INPUT_SAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
const INPUT: &str = "3299143-3378031,97290-131156,525485-660941,7606-10180,961703-1031105,6856273537-6856492968,403537-451118,5330-7241,274725-384313,27212572-27307438,926609-954003,3035-3822,161-238,22625-31241,38327962-38415781,778-1155,141513-192427,2-14,47639-60595,4745616404-4745679582,1296-1852,80-102,284-392,4207561-4292448,404-483,708177-776613,65404-87389,5757541911-5757673432,21-38,485-731,1328256-1444696,11453498-11629572,41-66,2147-3014,714670445-714760965,531505304-531554460,4029-5268,3131222053-3131390224";

fn parse_range(range: &str) -> (u64, u64) {
    if let Some((start_str, end_str)) = range.split_once("-") {
        let start = start_str.parse().unwrap_or(0);
        let end = end_str.parse().unwrap_or(0);

        return (start, end);
    }

    (0, 0)
}

fn sum_invalid_ids(range: (u64, u64)) -> u64 {
    let mut count = 0;
    for id in (range.0)..=(range.1) {
        if is_id_made_of_doubles(id) {
            count += id;
        }
    }

    count
}

fn is_id_made_of_doubles(id: u64) -> bool {
    let value = id.to_string();
    let len = value.len();

    if len % 2 == 0 {
        if &value[..len / 2] == &value[len / 2..] {
            return true;
        }
    }

    false
}

pub fn run() {
    // process input, split by comma
    let ranges = INPUT.split(",");

    let invalid_ids = ranges.fold(0, |acc, range| acc + sum_invalid_ids(parse_range(range)));

    println!("Invalid ids summed as {invalid_ids}");
}
