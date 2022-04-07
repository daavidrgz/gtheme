[1mdiff --git a/src/core/pattern/mod.rs b/src/core/pattern/mod.rs[m
[1mindex 4bb26b49..2fda0769 100644[m
[1m--- a/src/core/pattern/mod.rs[m
[1m+++ b/src/core/pattern/mod.rs[m
[36m@@ -132,11 +132,10 @@[m [mimpl Pattern {[m
 	}[m
 [m
 	fn get_patterns_from_path(path: &Path) -> Vec<PatternFile> {[m
[31m-		let path_name = path.as_os_str().to_string_lossy();[m
 		let entries = match fs::read_dir(path) {[m
 			Ok(dir) => dir,[m
 			Err(e) => {[m
[31m-				error!("Could not read directory |{}|: |{}|", path_name, e);[m
[32m+[m				[32merror!("Could not read directory |{}|: |{}|", path.display(), e);[m
 				return vec![][m
 			}[m
 		};[m
[36m@@ -146,7 +145,7 @@[m [mimpl Pattern {[m
 			let entry = match entry {[m
 				Ok(entry) => entry,[m
 				Err(e) => {[m
[31m-					error!("Error while reading entry from dir |{}|: |{}|", path_name, e);[m
[32m+[m					[32merror!("Error while reading entry from dir |{}|: |{}|", path.display(), e);[m
 					continue;[m
 				}[m
 			};[m
[36m@@ -159,6 +158,7 @@[m [mimpl Pattern {[m
 				}[m
 			};[m
 [m
[32m+[m			[32m//TODO: use path.display or handle utf-u errors?[m
 			let path = match entry.path().to_str() {[m
 				Some(path) => String::from(path),[m
 				None => {[m
[36m@@ -332,5 +332,11 @@[m [mmod tests{[m
 		};		[m
 	}[m
 [m
[32m+[m	[32m#[test][m
[32m+[m	[32mfn test_subpatterns(){[m
[32m+[m		[32mlet subpattern = "test.test2.subsubpattern";[m
[32m+[m		[32mdbg!(subpattern.split('.').collect::<Vec<&str>>());[m
[32m+[m	[32m}[m
[32m+[m
 [m
 }[m
