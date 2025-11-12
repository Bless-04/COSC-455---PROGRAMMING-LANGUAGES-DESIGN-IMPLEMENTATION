#include <iostream>
#include <string>
#include <vector>

using namespace std;

//
// Blessing Abumere
//

int main(void) {
    //                     Fundamental C++ types
    // There are several fundamental types in C++ including char, short, int
    // long, unsigned, float, double, long double, boolean (bool) and void.
    // We can use the sizeof() function to determine how much memory is
    // allocated for these various data types to better explore the concepts
    // of Chapter 6 and understand how and when memory is allocated (static vs.
    // dynamic). Following the example given for char, fill in the code for the
    // remaining fundamental types (if possible).

    cout << "Fundamental Types\n";
    cout << "    Type  " << "Size \n";
    cout << "    char: " << sizeof(char) << " bytes.\n";
    cout << "    int: " << sizeof(int) << " bytes.\n";
    cout << "    double: " << sizeof(double) << " bytes.\n";
    cout << "    long: " << sizeof(long) << " bytes.\n";
    cout << "    string: " << sizeof(string) << " bytes.\n";
    cout << "    bool: " << sizeof(bool) << " bytes.\n";

    //                         C++ Pointers
    // C++ allows for pointers to, among others, char, long and void data types.
    // We can again use the sizeof() function to determine how much memory is
    // allocated for these various data types to better explore the concepts
    // of Chapter 6 and understand how and when memory is allocated (static vs.
    // dynamic). As you did above, fill in the code to determine the memory size
    // allocated for these three pointer types.

    cout << "\nPointers\n";
    cout << "    Type  " << "Size \n";

    // Example for an integer pointer
    int i = 455;
    int* intPointer = &i;
    cout << "    int pointer: " << sizeof(intPointer) << " bytes.\n";

    char c = 'a';
    char* ptr_c = &c;
    cout << "    char pointer: " << sizeof(ptr_c) << " bytes.\n";

    long l = 1 << 30;
    long* ptr_l = &l;
    cout << "    long pointer: " << sizeof(ptr_l) << " bytes.\n";

    bool b = true;
    bool* ptr_b = &b;
    cout << "    bool pointer: " << sizeof(ptr_b) << " bytes.\n";

    string str = string();
    string* ptr_str = &str;
    cout << "    string pointer: " << sizeof(ptr_str) << " bytes.\n";

    //                        C++ Strings
    // Using the <string> library, we can explore the memory sizes allocated for
    // C++ strings using the sizeof() function. You should fill in code below to
    // experiment with how C++ allocates memory for various strings. For
    // example, you may want to compare setting the value of a string with
    // "josh" and getting its sizeof() compared with sizeof("josh"). You should
    // also provide code to determine when the memory size of the string is
    // determined and whether it can change or not.

    cout << "\nStrings\n";
    cout << "    Type  " << "Size \n";
    string s;
    s = "josh";
    cout << "    \"josh\": " << sizeof("josh") << " bytes.\n";
    cout << "    string(\"josh\"): " << sizeof(s) << " bytes.\n";
    cout << "    string(\"longer string\"): " << sizeof(string("longer string"))
         << " bytes.\n";
    cout << "    \"longer string\": " << sizeof("longer string") << " bytes.\n";

    //               C++ Enumerations
    cout << "\nEnumerations\n";
    cout << "    Type  " << "Size \n";

    enum weekday {
        monday = 1,
        tuesday = 2,
        wednesday = 3,
        thursday = 4,
        friday = 5,
        saturday = 6,
        sunday = 7
    };

    cout << "    enums: " << sizeof(weekday) << " bytes.\n"
         << "    weekday::monday: " << sizeof(weekday::monday) << " bytes.\n"
         << "    weekday::sunday: " << sizeof(weekday::sunday) << " bytes.\n";

    //                        C++ Arrays
    // In C++, you can declare arrays by providing a name and a size of the
    // array. The sizeof() function will again provide the amount of memory
    // allocated for the array. Experiment with different array sizes and types
    // to see how much memory is allocated, and when it is allocated
    // (declaration vs. instantiation).
    cout << "\nArrays\n";
    cout << "    Type  " << "Size \n";
    cout << "    int[10]:\t" << sizeof(int[10]) << " bytes.\n"
         << "    int[5]:\t" << sizeof(int[5]) << " bytes.\n"
         << "    double[10]:\t" << sizeof(double[10]) << " bytes.\n"
         << "    long[10]:\t" << sizeof(long[10]) << " bytes.\n"
         << "    int[10][10][10]:\t" << sizeof(int[10][10][10]) << " bytes.\n";

    //               C++ Standard Library Vectors
    // In C++, you can declare a vector as follows: vector<char> v(10); and
    // similarly determine its size using the sizeof() function. Experiment with
    // different vector sizes and types so that you can compare the differences
    // bewteen C++ arrays and vectors.

    cout << "\nVectors\n";
    cout << "    Type  " << "Size \n";
    cout << "    vector<int>(10):\t" << sizeof(vector<int>(10)) << " bytes.\n"
         << "    vector<int>(5):\t" << sizeof(vector<int>(5)) << " bytes.\n"
         << "    vector<double>(10):\t" << sizeof(vector<double>(10))
         << " bytes.\n"
         << "    vector<vector<vector<int>>>(10):\t"
         << sizeof(vector<vector<vector<int>>>(10)) << " bytes.\n"
         << "    vector<int>(2^20):\t" << sizeof(vector<int>(1 << 20))
         << " bytes.\n";
}