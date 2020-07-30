initSidebarItems({"enum":[["NewTaskMessage","Enum for all types of reported messages for a given Task, as per the `LaTeXML` convention One of \"invalid\", \"fatal\", \"error\", \"warning\" or \"info\""],["TaskMessage","Enum for all types of reported messages for a given Task, as per the `LaTeXML` convention One of \"invalid\", \"fatal\", \"error\", \"warning\" or \"info\""],["TaskStatus","An enumeration of the expected task statuses"]],"fn":[["generate_report","Generates a `TaskReport`, given the path to a result archive from a `CorTeX` processing job Expects a \"cortex.log\" file in the archive, following the `LaTeXML` messaging conventions"],["parse_log","Parses a log string which follows the `LaTeXML` convention (described at the Manual)"],["prepare_input_stream","Returns an open file handle to the task's entry"],["rand_in_range","Helper for generating a random i32 in a range, to avoid loading the rng crate + boilerplate"],["random_mark","Generate a random integer useful for temporary DB marks"],["utf_truncate","Utility functions, until they find a better place"]],"struct":[["LOADING_LINE_REGEX","\"(Loading... file\" message regex"],["TaskProgress","In-progress task, with dispatch metadata"],["TaskReport","Completed task, with its processing status and report messages"]]});