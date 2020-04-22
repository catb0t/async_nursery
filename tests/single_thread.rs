// Tested:
//
// ✔ Basic usage within function.
// ✔ A nursery passed in to a function that uses it to spawn.
//
mod common;

use common::{ import::*, DynResult };


// Basic usage within function.
//
#[test] fn in_method_local() -> DynResult
{
	let exec              = TokioCt::try_from( &mut Builder::new() )?;
	let (nursery, output) = Nursery::new_local( exec.clone() )?;

	nursery.nurse( async { 5 + 5 } )?;
	nursery.nurse( async { 5 + 5 } )?;
	drop(nursery);

	let sum = exec.block_on( output.fold( 0, |acc, x| async move { acc + x } ) );

	assert_eq!( 20, sum );

	Ok(())
}


// A nursery passed in to a function that uses it to spawn.
//
#[test] fn outlive_method_local() -> DynResult
{
	fn outlive( nursery: &Nursery<TokioCt, usize> ) -> DynResult
	{
		nursery.nurse( async { 5 + 5 } )?;
		nursery.nurse( async { 5 + 5 } )?;

		Ok(())
	}

	let exec              = TokioCt::try_from( &mut Builder::new() )?;
	let (nursery, output) = Nursery::new_local( exec.clone() )?;

	outlive( &nursery )?;
	drop(nursery);

	let sum = exec.block_on( output.fold( 0, |acc, x| async move { acc + x } ) );

	assert_eq!( 20, sum );

	Ok(())
}
