#![warn(clippy::all, clippy::as_conversions)]
#![allow(clippy::too_many_arguments, clippy::blocks_in_if_conditions)]

use std::io;
use std::path::PathBuf;

use cursive::backends::crossterm;
use cursive::CursiveRunnable;
use cursive_buffered_backend::BufferedBackend;

use git_record::Recorder;
use git_record::{FileContent, Hunk, HunkChangedLine, RecordError, RecordState};

fn main() {
    let files = vec![
        (
            PathBuf::from("foo/bar"),
            FileContent::Text {
                hunks: vec![
                    Hunk::Unchanged {
                        contents: std::iter::repeat("this is some text".to_string())
                            .take(20)
                            .collect(),
                    },
                    Hunk::Changed {
                        before: vec![
                            HunkChangedLine {
                                is_selected: true,
                                line: "before text 1".to_string(),
                            },
                            HunkChangedLine {
                                is_selected: true,
                                line: "before text 2".to_string(),
                            },
                        ],
                        after: vec![
                            HunkChangedLine {
                                is_selected: true,
                                line: "after text 1".to_string(),
                            },
                            HunkChangedLine {
                                is_selected: false,
                                line: "after text 2".to_string(),
                            },
                        ],
                    },
                    Hunk::Unchanged {
                        contents: vec!["this is some trailing text".to_string()],
                    },
                ],
            },
        ),
        (
            PathBuf::from("baz"),
            FileContent::Text {
                hunks: vec![
                    Hunk::Unchanged {
                        contents: vec![
                            "Some leading text 1".to_string(),
                            "Some leading text 2".to_string(),
                        ],
                    },
                    Hunk::Changed {
                        before: vec![
                            HunkChangedLine {
                                is_selected: true,
                                line: "before text 1".to_string(),
                            },
                            HunkChangedLine {
                                is_selected: true,
                                line: "before text 2".to_string(),
                            },
                        ],
                        after: vec![
                            HunkChangedLine {
                                is_selected: true,
                                line: "after text 1".to_string(),
                            },
                            HunkChangedLine {
                                is_selected: true,
                                line: "after text 2".to_string(),
                            },
                        ],
                    },
                    Hunk::Unchanged {
                        contents: vec!["this is some trailing text".to_string()],
                    },
                ],
            },
        ),
    ];
    let record_state = RecordState { files };

    // TODO: let user select backend
    // let mut siv = cursive::default();
    let siv = CursiveRunnable::new(|| -> io::Result<_> {
        // Use crossterm to ensure that we support Windows.
        let crossterm_backend = crossterm::Backend::init()?;
        Ok(Box::new(BufferedBackend::new(crossterm_backend)))
    });
    let siv = siv.into_runner();

    let recorder = Recorder::new(record_state);
    let result = recorder.run(siv);
    let RecordState { files: result } = match result {
        Ok(result) => result,
        Err(RecordError::Cancelled) => todo!("Cancelled"),
    };
    for (path, file_hunks) in result {
        println!("Path {}", path.display());
        let (selected, _unselected) = file_hunks.get_selected_contents();
        print!("{}", selected);
    }
}
