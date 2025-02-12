use std::error::Error;

use bladeink::{story::Story, story_error::StoryError, value_type::ValueType};

mod common;

#[test]
fn operations_test() -> Result<(), StoryError> {
    let json_string = common::get_json_string("inkfiles/misc/operations.ink.json").unwrap();
    let mut story = Story::new(&json_string)?;

    assert_eq!(
        "neg:-3\nmod:1\npow:27\nfloor:3\nceiling:4\nint:3\nfloat:1\n",
        &story.continue_maximally()?
    );

    Ok(())
}

#[test]
fn read_counts_test() -> Result<(), StoryError> {
    let json_string = common::get_json_string("inkfiles/misc/read-counts.ink.json").unwrap();
    let mut story = Story::new(&json_string)?;

    assert_eq!(
        "Count start: 0 0 0\n1\n2\n3\nCount end: 3 3 3\n",
        &story.continue_maximally()?
    );

    Ok(())
}

#[test]
fn turns_since_test() -> Result<(), StoryError> {
    let json_string = common::get_json_string("inkfiles/misc/turns-since.ink.json").unwrap();
    let mut story = Story::new(&json_string)?;

    assert_eq!("0\n0\n", &story.continue_maximally()?);
    story.choose_choice_index(0)?;
    assert_eq!("1\n", &story.continue_maximally()?);

    Ok(())
}

/**
 * Issue: https://github.com/bladecoder/blade-ink/issues/15
 */
#[test]
fn issue15_test() -> Result<(), StoryError> {
    let json_string = common::get_json_string("inkfiles/misc/issue15.ink.json").unwrap();
    let mut story = Story::new(&json_string)?;

    assert_eq!("This is a test\n", story.cont()?);

    while story.can_continue() {
        // println!(story.buildStringOfHierarchy());
        let line = &story.cont()?;

        if line.starts_with("SET_X:") {
            story.set_variable("x", &ValueType::Int(100))?;
        } else {
            assert_eq!("X is set\n", line);
        }
    }

    Ok(())
}

#[test]
fn newlines_with_string_eval_test() -> Result<(), Box<dyn Error>> {
    let json_string = common::get_json_string("inkfiles/misc/newlines_with_string_eval.ink.json")?;
    let mut story = Story::new(&json_string)?;

    assert_eq!("A\nB\nA\n3\nB\n", &story.continue_maximally()?);

    Ok(())
}
