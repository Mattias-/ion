import sys
import os
import subprocess


GREEN = '\033[92m'
RED = '\033[91m'
RESET = '\033[0m'

def print_c(color, s):
    print(color + s + RESET)

def main(args):
    neg_test = args[1].lower() == "--neg"
    testpath = args[2]
    executable = args[3]

    total_tests = 0
    passed_tests = 0

    for file in get_files(testpath):
        name, _ = os.path.splitext(file)
        total_tests += 1
        p = subprocess.Popen([executable, testpath+file],
                             stdin=subprocess.PIPE,
                             stdout=subprocess.PIPE,
                             stderr=subprocess.PIPE)
        (out, err) = p.communicate()

        if (p.returncode == 0) == (not neg_test):
            print_c(GREEN, "Passed: " + testpath + file)
            passed_tests += 1
        else:
            print_c(RED, "Failed: " + file)

#        outfile = testpath + file + ".out"
#        if os.path.isfile(outfile):
#            outdata = open(outfile).read()
#            if out == outdata:
#                print "Passed:", GREEN+file+RESET
#                passed_tests += 1
#            else:
#                print "Output when running"+RED, file, RESET
#                print RED+out_res+RESET
#                print "Does not match:"
#                print outdata
#        else:
#            print "No output file for", file


    if total_tests == passed_tests:
        col = GREEN
    else:
        col = RED
    print_c(col, "Passed: %d of %d in %s" % (passed_tests, total_tests,
                                             testpath))

def get_files(path):
    a = os.listdir(path)
    files = filter(lambda x: os.path.splitext(x)[1] == ".ion", a)
    files2 = sorted(files)
    return files2

if __name__ == '__main__':
    main(sys.argv)
