// Written Fall 2022 by Prof. Raymond Klefstad to test output for UCI ICS 141 Homework assignment 8 & 9

#include <iostream>
#include <fstream>
#include <sstream>
#include <map>

using namespace std;

const int NUM_COPIES_PER_FILE = 2;
int NUM_USERS = 1;
const int NUM_FILES_PER_USER = 10;
const int LINES_PER_FILE = 5;
char user_names[] = {'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'};

int files_per_printer[100] = {0};
int num_printers = 0;
int total_files_printed = 0;

map<string,int> copies_per_file;


void error(string msg)
{
    cout << "ERROR: " << msg << endl;
}


string parse_file_from_printer(ifstream & in)
{
    string file_name;
    string data;

    for (int i=0; i<LINES_PER_FILE; ++i)
    {
        in >> file_name;
        if (!in) break;
        in >> data;
        if (!in) break;
        // cout << "TRACE: " << file_name << " " << data << endl;
        if ((data[0]-'0') != i)
            error("file not complete");
    }
    return file_name;
}


void parse_printer_file(int printer_id, char *printer_name)
{
    ifstream in(printer_name);
    string file_name;
    while (file_name = parse_file_from_printer(in), in)
    {
        // cout << "TRACE: " << "read file named " << file_name << endl;
        ++copies_per_file[file_name];
        ++files_per_printer[printer_id];
        ++total_files_printed;
    }
    in.close();
    ++num_printers;
}


void verify_output_is_correct(int n_printers)
{
    int average = total_files_printed / n_printers;
    int min_files_per_printer = average * 0.90;

    for (int i=0; i<n_printers; ++i)
    {
        // cout << "files on printer " << i << ": " << files_per_printer[i] << endl;
        // cout << "min files per printer: " << min_files_per_printer << endl;
        if (files_per_printer[i] <= min_files_per_printer)
            error("Not enough files on a printer");
    }
    int files_expected = NUM_USERS * NUM_COPIES_PER_FILE * NUM_FILES_PER_USER;

    if (total_files_printed != files_expected)
    {
        error("Not enough files printed");
        cout << "Expecting " << files_expected << " got " << total_files_printed << endl;
    }
    for (auto x : copies_per_file)
        if (x.second != NUM_COPIES_PER_FILE)
            cout << "ERROR: incorrect number of copies of file, " << " got " << x.second <<
                   " but should be " << NUM_COPIES_PER_FILE << endl;
}


void print_file_statistics()
{
    cout << "Total files printed: " << total_files_printed << endl;
    cout << "Number of PRINTERS: " << num_printers << endl;
    for (int i=0; i<num_printers; ++i)
        cout << "Printer " << i << " has " << files_per_printer[i] << endl;
}


int main(int argc, char *argv[])
{
    int n_printers = argc-2;
    NUM_USERS = atoi(argv[1]);

    for (int i=2; i<argc; ++i)
        parse_printer_file(i-2, argv[i]);
    print_file_statistics();
    verify_output_is_correct(n_printers);
    return 0;
}
