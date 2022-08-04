trait EmergencyMeeting {
    fn report_it(&self) -> String;
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str // generics in
                                                                                  // same angle
                                                                                  // brackets
    where T: EmergencyMeeting
{
    println!("Announcement! {}", ann.report_it());
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

struct AmongUs<'a, 'b> {
    location: &'b str,
    color: &'a str,
    kill_count: i32,
    guilt: bool,
}

impl<'a, 'b> EmergencyMeeting for AmongUs<'a,'b> {
    fn report_it(&self) -> String {
        let mut base_string = String::from(self.color);
        base_string.push_str(" was at ");
        base_string.push_str(self.location);
        if self.guilt && self.kill_count > 1 {
            base_string.push_str(" and seems sus.");
        }
        else { 
            base_string.push_str(" and is chilling."); 
        }
        base_string
    }
}

fn main() {
    let this_str: &str = "String literals are cool";
    let another_str: String = String::from("but so are string types o_O");
    let the_imposter = AmongUs { location: "med bay", color: "red", kill_count: 2, guilt: true };
    let longest = longest_with_an_announcement(this_str, another_str.as_str(), the_imposter);

    println!("The longest string was {}", longest);
}
