extern crate pgs_files;
extern crate pwhash   ;
extern crate privdrop ;

use std::process::exit;
use std::process::Command;

use pgs_files::shadow;
use pwhash   ::unix;
use privdrop ::PrivDrop;



fn main()
{
	// let credentials = "/dev/shm/cred";

	let user = "test";
	let pass = "test";

	// Get shadow entry
	//
	let option = shadow::get_entry_by_name( user );
	let entry;


	if None == option
	{
		print!( "User does not exist" );
		exit( 1 );
	}

	else { entry = option.unwrap(); }


	// Compare pass with shadow entry
	//
	if ! unix::verify( pass, &entry.passwd )
	{
		print!( "Wrong password" );
		exit( 2 );
	}


	// We're good, user authenticated.
	//
	// Change uid
	//
	let dropped = PrivDrop::default().user( user ).apply();

	print!("{:?}\n", dropped);

	// Run command
	//
	let output = Command::new( "id" ).output()

		.unwrap_or_else( |e|
		{
	   	panic!( "failed to execute id: {}", e )
		})
	;

	let stdout = std::str::from_utf8( &output.stdout ).unwrap();

	print!( "{:?}", stdout );
}
