extern crate pgs_files;
extern crate pwhash   ;
extern crate privdrop ;
extern crate users    ;

use std::process::exit;
use std::process::Command;
use std::os::unix::process::CommandExt;
use std::env;

use pgs_files::shadow;

use pwhash   ::unix;
use privdrop ::PrivDrop;
use users    ::get_current_uid;


fn main()
{
	let args: Vec<String> = env::args().collect();

	if args.len() != 3 { usage(); exit( 1 ) }

	let user = &args[1];
	let cmd  = &args[2];
	let pass = askpass( user );

	// Get shadow entry
	//
	let option = shadow::get_entry_by_name( user );
	let entry  = option.expect( "User does not exist" );

	// Compare pass with shadow entry
	//
	if ! unix::verify( &pass, &entry.passwd )
	{
		print!( "Wrong password" );
		exit( 2 );
	}


	// We're good, user authenticated.
	//
	// Drop privileges
	//
	PrivDrop::default().user( user ).apply()

		.unwrap_or_else( |e| { panic!( "Failed to drop privileges to user: {}, {}", user, e ) } )
	;


	// Run command
	//
	let output = Command::new( "bash" ).arg( "-c" ).arg( cmd ).output()

		.unwrap_or_else( |e| { panic!( "failed to execute {}: {}", cmd, e ) } )
	;

	let stdout = std::str::from_utf8( &output.stdout ).unwrap();

	print!( "{:?}", stdout );
}



fn askpass( user: &str ) -> String
{
	// Run zenity
	//
	let output = Command::new( "zenity" )

		.args( &[ "--password", "--title", &format!( "'Password for user {:}'", user ) ] )
		.uid( get_current_uid() )
		.output()
		.unwrap_or_else( |e| { panic!( "Failed to execute zenity: {}", e ) } )
	;


	// Remove trailing newline
	//
	std::str::from_utf8( &output.stdout[ ..&output.stdout.len()-1 ] )

		.expect( "Password from returned from zenity contains invalid utf." )
		.to_string()
}


fn usage()
{
	print!(
"
ou (other user) allows you to run a command as another user when you are yourself unprivileged. You will need to have the password of the other user. A zenity dialog will ask you for the password. This allows to create .desktop launcher which will run commands as a different user without needing the end user to use the terminal.

The ou binary must be owned by root and have the setuid bit set.

Usage: ou user cmd

");

}
