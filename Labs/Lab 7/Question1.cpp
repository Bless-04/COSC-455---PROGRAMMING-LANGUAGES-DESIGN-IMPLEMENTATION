#include <iostream>
#include <string>
#include <vector>

using namespace std;

  //
  // YOUR NAME: 
  //

int main(void)
{
  //                     Fundamental C++ types  
  // There are several fundamental types in C++ including char, short, int
  // long, unsigned, float, double, long double, boolean (bool) and void. 
  // We can use the sizeof() function to determine how much memory is
  // allocated for these various data types to better explore the concepts
  // of Chapter 6 and understand how and when memory is allocated (static vs.
  // dynamic). Following the example given for char, fill in the code for the
  // remaining fundamental types (if possible). 
  
    cout <<"Fundamental Types\n";	 
	cout <<"    Type  " << "Size \n";
	cout <<"    char: " << sizeof(char) << " bytes.\n";	   
	cout <<"    int: " << sizeof(int) << " bytes.\n";	   	   
	
  //                         C++ Pointers
  // C++ allows for pointers to, among others, char, long and void data types.
  // We can again use the sizeof() function to determine how much memory is
  // allocated for these various data types to better explore the concepts
  // of Chapter 6 and understand how and when memory is allocated (static vs.
  // dynamic). As you did above, fill in the code to determine the memory size
  // allocated for these three pointer types.
      
	cout <<"\nPointers\n";
	cout <<"    Type  " << "Size \n";
	
	// Example for an integer pointer
	int i = 455;
	int *intPointer = &i;
	cout <<"    int pointer: " << sizeof(intPointer) << " bytes.\n";	
	
  //                        C++ Strings
  // Using the <string> library, we can explore the memory sizes allocated for
  // C++ strings using the sizeof() function. You should fill in code below to
  // experiment with how C++ allocates memory for various strings. For example,
  // you may want to compare setting the value of a string with "josh" and
  // getting its sizeof() compared with sizeof("josh"). You should also provide
  // code to determine when the memory size of the string is determined and 
  // whether it can change or not. 
  
	cout <<"\nStrings\n";
	cout <<"    Type  " << "Size \n";
	string s; 
	s = "josh";
	cout <<"    string: " << sizeof(s) << " bytes.\n";
	cout <<"    string: " << sizeof("josh") << " bytes.\n";
	
  //               C++ Enumerations
	cout <<"\nEnumerations\n";
	cout <<"    Type  " << "Size \n";
	
	enum weekday {monday = 1, tuesday = 2, wednesday = 3, thursday = 4, friday = 5, saturday = 6, sunday = 7};
	cout <<"    enums: " << sizeof(weekday) << " bytes.\n";
	
  //                        C++ Arrays
  // In C++, you can declare arrays by providing a name and a size of the array.
  // The sizeof() function will again provide the amount of memory allocated for
  // the array. Experiment with different array sizes and types to see how much memory 
  // is allocated, and when it is allocated (declaration vs. instantiation). 
  	cout <<"\nArrays\n";
	cout <<"    Type  " << "Size \n";
		
	 
  //               C++ Standard Library Vectors
  // In C++, you can declare a vector as follows: vector<char> v(10); and similarly
  // determine its size using the sizeof() function. Experiment with different 
  // vector sizes and types so that you can compare the differences bewteen C++ 
  // arrays and vectors. 

	cout <<"\nVectors\n";
	cout <<"    Type  " << "Size \n";
	
}