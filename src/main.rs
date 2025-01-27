extern crate astronomicals;
extern crate printwrap;
use std::{env,fs::File,path::Path,process};
use serde::{Deserialize, Serialize};
use chrono::{Weekday,Local,LocalResult,Datelike,NaiveDateTime,offset::TimeZone};
use log::*;
use astronomicals::*;
use rand::Rng;
	

#[derive(Serialize, Deserialize)]
struct Condition
{
	field: String,
	conditional: String,
	value: i32,
}

#[derive(Serialize, Deserialize)]
struct Selector 
{
	comment: Vec<String>,
	conditions: Vec<Condition>,
	emojis: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct SeasonalemojiJSON
{
	selectors: Vec<Selector>,
}

fn usage()
{
	printwrap::print_wrap(5,0,"Seasonalemoji prints an emoji (or any other unicode character or characters) to stdout appropriate to the current date as represented in the configfile. The default config file is in /usr/local/share/seasonalemoji/seasonalemoji.json. If no config is found, the current directory will also be checked.");
	printwrap::print_wrap(5,0,"");
	printwrap::print_wrap(5,0,"Usage:");
	printwrap::print_wrap(5,0,"    seasonalemoji [options]");
	printwrap::print_wrap(5,0,"Options:");
	printwrap::print_wrap(5,19,"    -h             Print this usage message.");
	printwrap::print_wrap(5,19,"    --date <date>  Use the date in <date> rather than the system date. <date> must be in the form of \"%Y-%m-%d-%H:%M\" (year-month-day-hour:minute)");
	printwrap::print_wrap(5,19,"    -f <file>      Read configuration from <file> rather than the default config file location.");
	printwrap::print_wrap(5,19,"    --why          Prints the comment for the chosen selector after printing the emoji.");
	printwrap::print_wrap(5,0,"");
	printwrap::print_wrap(5,0,"The config file is a JSON file comprised primarly of an array of \"selectors\" each of which will \"select\" a specific date or date range and specify a list of emojis or other characters (any unicode characters will be valid).");
	printwrap::print_wrap(5,0,"");
	printwrap::print_wrap(5,0,"When seasonalemoji is run, the current date is checked against the selectors and the last one which positively matches will be selected. One of the emojis listed for that selector will be randomly chosen and printed on stdout.");
	printwrap::print_wrap(5,0,"");
	printwrap::print_wrap(5,0,"\"selectors\" is an array of elements. Each selector will contain one or more \"conditions\" that, when matched, will select one or more specified emojis. Each condition has three attributes: field, conditional, value.");
	printwrap::print_wrap(5,0,"");
	printwrap::print_wrap(5,0,"Available fields are:");
	printwrap::print_wrap(5,15,"    DOY        (day of year, 1-366)");
	printwrap::print_wrap(5,15,"    MOY        (month of year, January=1, December=12)");
	printwrap::print_wrap(5,15,"    DOM        (day of month, 1-31)");
	printwrap::print_wrap(5,15,"    DOW        (day of week, 1=Sunday, 7=Saturday)");
	printwrap::print_wrap(5,15,"    YEAR       (4 digit year)");
	printwrap::print_wrap(5,15,"    SOL        (solstice is 1 on the Summer or Winter solstices)");
	printwrap::print_wrap(5,15,"    EQNX       (equinox is 1 on the Spring or Autumn equinoxes)");
	printwrap::print_wrap(5,0,"");
	printwrap::print_wrap(5,0,"The conditionals that be applied to the field are:");
	printwrap::print_wrap(5,15,"    =          equal to");
	printwrap::print_wrap(5,15,"    !=         not equal to");
	printwrap::print_wrap(5,15,"    <          less than");
	printwrap::print_wrap(5,15,"    <=         less than or equal to");
	printwrap::print_wrap(5,15,"    >          greater than");
	printwrap::print_wrap(5,15,"    >=         greater than or equal to");
	printwrap::print_wrap(5,0,"");
	printwrap::print_wrap(5,0,"The value is an interger number that the field is compared against.");
	printwrap::print_wrap(5,0,"");
	printwrap::print_wrap(5,0,"If *all* conditions in a single selector are true, the selector is matched. The selectors in seasonalemoji.json are evaluated from first to last and later selectors will override their predecessors. If you need a selector where either of two conditions could be true, then simply create two separate selectors. For example if you wanted different emojis for the Spring Equinox versus the Fall Equinox, then two separate selectors would be created. Both checking EQNX=1, and one checking MOY=6 and the other MOY=9.");
	printwrap::print_wrap(5,0,"");
	printwrap::print_wrap(5,0,"The actual emojis are listed in an array. If more than one emoji is present and matched, one will be randomly selected. Each emoji can actually be more than a single emoji, say 💘💌, if that is what you want. ");
	printwrap::print_wrap(5,0,"");
	printwrap::print_wrap(5,0,"format of seasonalemoji.json:");
	printwrap::print_wrap(5,0,"{");
	printwrap::print_wrap(5,0,"\"selectors\": [");
	printwrap::print_wrap(5,0,"    {");
	printwrap::print_wrap(5,20,"        \"comment\": [\"a comment describing the selector here\"],");
	printwrap::print_wrap(5,20,"        \"conditions\":[");
	printwrap::print_wrap(5,20,"        {");
	printwrap::print_wrap(5,20,"            \"field\": \"<field option here>\",");
	printwrap::print_wrap(5,20,"            \"conditional\": \"<conditional option here>\",");
	printwrap::print_wrap(5,20,"            \"value\": <number here>");
	printwrap::print_wrap(5,20,"        }[,<additional conditions>]");
	printwrap::print_wrap(5,20,"        \"emojis\": [\"<emoji>\"[, \"<additional emoji>\"]]");
	printwrap::print_wrap(5,20,"    }[, <additional selectors>]");
	printwrap::print_wrap(5,0,"}");
	printwrap::print_wrap(5,0,"");
	printwrap::print_wrap(5,0,"Return codes:");
	printwrap::print_wrap(5,8,"    1 - failed to read config file");
	printwrap::print_wrap(5,8,"    3 - error parsing config file");
	printwrap::print_wrap(5,8,"    0 - success");
	printwrap::print_wrap(5,8,"    4 - failed to parse config file with no error");
	printwrap::print_wrap(5,8,"    5 - program somehow managed to not complete successfully or with a known error.");
	printwrap::print_wrap(5,0,"");
}


fn walk(emojis:&SeasonalemojiJSON)
{
	println!("Walking SeasonalemojiJSON...");
	for i in &emojis.selectors 
	{
		println!("Comment:");
		for n in &i.comment
		{
			println!("\t\"{}\"", n);
		}
		//println!("Emojis:\"{}\"",i.emojis);
		println!("\tConditions:");
		for w in &i.conditions
		{
			println!("\t\t\"{}\" {} \"{}\"", w.field,w.conditional,w.value);
		}
		println!("\tEmojis:");
		for n in &i.emojis
		{
			println!("\t\t\"{}\"", n);
		}
	}
	println!("Done walking config.");
	process::exit(2);
}

fn load_config(file_path: &str) -> SeasonalemojiJSON
{
	debug!("Json_file_path: \"{}\"", file_path);

	let json_file_path = Path::new(file_path);
	let file = match File::open(json_file_path) 
	{
		Err(error) => {error!("Could not open file \"{}\"\n{}", file_path,error);process::exit(1)},
		Ok(file) => file,
	};
	let emojis= match serde_json::from_reader(file)
	{
		Err(error) => {error!("Error reading file \"{}\"\n{}",file_path, error);process::exit(3)},
		Ok(emojis) => emojis,
	};
	return emojis;
}

fn check_conditions( conditions: &Vec<Condition>, month_of_year: i32, day_of_month: i32, 
					day_of_year:i32, day_of_week:i32, year:i32, solstice:i32, equinox:i32, easter:i32) -> bool
{
	let mut matched=true;
	for c in conditions
	{
		// assign to field_value whichever part of the date parameters is indicated
		// by the "field" field in the conditon
		let field_value = match c.field.as_str() 
			{
				"MOY" => month_of_year,
				"DOM" => day_of_month,
				"DOY" => day_of_year,
				"DOW" => day_of_week,
				"YEAR" => year,
				"SOL" => solstice,
				"EQNX" => equinox,
				"ESTR" => easter,
				_ => -1
			};
		// assign to lmatched the result of a logical comparison of the field_value (part of the date)
		// and the value field in the conditon using the appropriate logical comparison as indicated
		// by the conditional field.
		let lmatched = match c.conditional.as_str()
		{
			"=" => field_value == c.value,
			"!=" => field_value != c.value,
			"<" => field_value < c.value,
			"<=" => field_value <= c.value,
			">" => field_value > c.value,
			">=" => field_value >= c.value,
			_ => false
		};
		matched = matched & lmatched;
		debug!("Condition: {} {} {}  ({})", field_value, c.conditional, c.value, lmatched);
		if !matched 
		{
			//as soon as matched is false, we can quit checking 
			break;
		}
	}
	matched
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let start=1;
	let end=args.len();
	let mut debug = false;
	let mut verbose = log::Level::Info; // default log level of INFO
	let mut json_file_path = "/usr/local/share/seasonalemoji/seasonalemoji.json";
	let mut skip_argument=false;
	let mut do_walk=false;
	let mut fakedate:&str = "";
	let mut today=Local::now();
	let mut parse_fake_date=false;
	let mut why=false;
	for i in start..end
	{
		if skip_argument
		{
			skip_argument = false;
		}
		else
		{
			match args[i].as_ref()
			{
				"-h" | "--help" =>
					{
					usage();
					}
				"-f" =>
					{
						if (i+1) < end
						{
							json_file_path=&args[i+1];
							skip_argument = true;
						}
					}
				"-c"|"--configtest" =>
					{
						do_walk=true;
					}
				"--date" =>
					{
						if (i+1) < end
						{
							fakedate=&args[i+1];
							parse_fake_date=true;
							skip_argument=true;
						}
					}
				"-v" =>
					{
						verbose = log::Level::Debug;
						debug=true;
					} 
				"-vv" =>
					{
						verbose = log::Level::Trace;
						debug=true;
					}
				"--why"=>
					{
						why=true;
					}
				_ =>
					{
						println!("Unknown argument \"{}\".",args[i]);
						usage();
					}

			}
		}
	}
	match stderrlog::new().module(module_path!()).verbosity(verbose).init()
	{
		Ok(l)=> l,
		Err(e) =>{println!("Failed to create stderr logger:{}",e)},
	}

	if parse_fake_date
	{
		trace!("Parsing date \"{}\"", fakedate);
		let fake_today = match NaiveDateTime::parse_from_str(fakedate, "%Y-%m-%d-%H:%M")
		{
			Err(error) => {error!("Error parsing date supplied \"{}\"\n{}",fakedate,error);process::exit(5)},
			Ok(fake_today) => fake_today,
		};
		today = match Local.from_local_datetime(&fake_today)
			{
				LocalResult::None => {error!("Failed to create fake date from {}",fake_today);process::exit(5)},
				LocalResult::Single(date_time) => date_time,
				LocalResult::Ambiguous(_date_time1, date_time2) => date_time2,
			};
		trace!("Parsed today (from fake date):{}", today);
	}
	else
	{
		trace!("Parsed today:{}", today);
	}

	let year:i32 = today.year();
	let month_of_year:u32 = today.month();
	let day_of_month:u32 = today.day();
	let day_of_year:u32=today.ordinal();
	let day_of_week_wd:Weekday=today.weekday();
	let day_of_week = day_of_week_wd.number_from_sunday();

	let ephemeris = calculateEphermeris(year, month_of_year as i32, day_of_month as i32, 0, debug);

	trace!("Ephemeris:");
	trace!("\tmarch_equinox:    {}", ephemeris.march_equinox);
	trace!("\tjune_solstice:    {}", ephemeris.june_solstice);
	trace!("\tseptember_equinox:{}", ephemeris.september_equinox);
	trace!("\tdecember_solstice:{}", ephemeris.december_solstice);
	trace!("\tsolstice:         {}", ephemeris.solstice);
	trace!("\tequinox:          {}", ephemeris.equinox);
	trace!("\teaster:           {}", ephemeris.easter);
	//let hour:u32 = today.hour();
	//let minute:u32 = today.minute();
	//let seconds:i64 = today.timestamp();


	let emojis = load_config(json_file_path);

	if do_walk
	{
		walk(&emojis);
	}

	let mut final_emoji = " ";
	let mut rng = rand::thread_rng();
	let mut comment = String::from("");
	for e in &emojis.selectors 
	{
		let solstice:i32 = if ephemeris.solstice { 1} else { 0};
		let equinox:i32 =  if ephemeris.equinox { 1} else { 0};
		let easter:i32 =   if ephemeris.easter {1} else {0};
		for c in &e.comment
		{
			debug!("{}", c);
		}
		if check_conditions(&e.conditions, month_of_year as i32, day_of_month as i32, day_of_year as i32, day_of_week as i32,
								year, solstice, equinox, easter)
		{
			let count = e.emojis.len();
			let rn: u16 = rng.gen();
			let index:usize = rn as usize % count;
			final_emoji = e.emojis[index].as_str();
			if why
			{
				comment=format!("{:?}",e.comment);
			}
		}
	}
	if why
	{
		println!("{} {}", final_emoji,comment);
	}
	else
	{
		println!("{}", final_emoji);
	}
}

