#!/bin/bash/python 

import sys
import os
import time
import re
import subprocess


kSolutionDir=os.path.join("src", "solution")
kProblemDir=os.path.join("src", "problem")

def eprint(msg):
    print>> sys.stderr, ("Error: {}".format(msg))

def test_solution(qid):
    """Invoke cargo test on the given problem
    """
    prob_name_pattern = r"p{0:04d}([a-zA-Z0-9_])*.rs$".format(qid)
    prob_filename = ""
    for filename in os.listdir(kProblemDir):
        if re.match(prob_name_pattern, filename):
            prob_filename = filename
            break

    if prob_filename == "":
        eprint("Fail to find Problem {}".format(qid))
        return 1

    filename_no_suffix = prob_filename.split(".")[0]
    command = "bash -c \"cargo test problem::{}::tests::test_{} -- --exact --nocapture\"".format(filename_no_suffix, qid)
    print command
    os.system(command)

    return 0


def init_problem(qid):
    """Init the problem without solution but with tests
    """
    # Check problem not exists
    prob_name_pattern = r"p{0:04d}(\D)*.rs$".format(qid)
    for filename in os.listdir(kProblemDir):
        if re.match(prob_name_pattern, filename):
            eprint("Problem {} already exists.".format(qid))
            return 1


    # Remove the corresponding line in problem/mod.rs
    # Otherwise, later 'cargo run' cmd may fail. 
    prob_mod_path = os.path.join(kProblemDir, "mod.rs")
    with open(prob_mod_path) as f:
        mod_lines = f.readlines()

    os.remove(prob_mod_path)

    with open(prob_mod_path, "w") as f:
        f.write("")
        for mod_line in mod_lines:
            if not mod_line.startswith("mod p{0:04d}".format(qid)):
                f.write(mod_line)

    # Invoke 'cargo run' to fetch the problem from Leetcode
    command = "bash -c \"cargo run <<< {}\"".format(qid)
    os.system(command)

    # Check solution exists
    sol_name_pattern = r"^s{0:04d}(\D)*.rs$".format(qid)
    solution_path = ""
    for filename in os.listdir(kSolutionDir):
        # print filename
        if re.match(sol_name_pattern, filename):
            solution_path = os.path.join(kSolutionDir, filename)
            break
        else:
            continue

    if solution_path == "":
        eprint("Solution {} not found in {}. No update on problem tests.".format(qid, kSolutionDir))
        return 0

    # Get problem_lines and get line number of tests session
    problem_path = ""
    for filename in os.listdir(kProblemDir):
        if re.match(prob_name_pattern, filename):
            problem_path = os.path.join(kProblemDir, filename)
            break
    
    if problem_path == "":
        eprint("Fail to download Problem {}".format(qid))
        return 1

    #  Identify the starting line number of the test section in problem
    test_line_pattern = "#[cfg(test)]"
    prob_test_line_num = -1
    with open(problem_path) as f:
        prob_lines = f.readlines()
    
    for i, prob_line in enumerate(prob_lines):
        if test_line_pattern in prob_line:
            prob_test_line_num = i
    
    if prob_test_line_num == -1:
        eprint("Fail to locate test section in {}".format(problem_path))
        return 1


    #  Identify the starting line number of the test section in solution
    test_line_pattern = "#[cfg(test)]"
    sol_test_line_num = -1
    with open(solution_path) as f:
        solution_lines = f.readlines()
    
    for i, solution_line in enumerate(solution_lines):
        if test_line_pattern in solution_line:
            sol_test_line_num = i
    
    if sol_test_line_num == -1:
        eprint("Fail to locate test section in {}".format(solution_path))
        return 1

    # Concat the non-test section in problem and test section in solution
    with open(problem_path, "w") as f:
        f.writelines(prob_lines[0:prob_test_line_num] + solution_lines[sol_test_line_num:])
    
    return 0

def main():
    # <= to exclude the count of sys.argv[0], the script name. 
    if len(sys.argv) <= 2:
        print("Insufficient parameters, expecting at least {}, but actual {}".format(2, len(sys.argv)-1))
        print("python helper.py [init/test] [problem_id]")
        return 1
    mode = sys.argv[1]
    qid = int(sys.argv[2])
    if mode == "init": 
        return init_problem(qid)
    elif mode == "test":
        return test_solution(qid)
    else:
        print("Unrecognized mode {}".mode)
        return 1
    


if __name__ == "__main__":
    sys.exit(main())