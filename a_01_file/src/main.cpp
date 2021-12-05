#include <iostream>
#include <fstream>
#include <string>
using namespace std;

int main () {
   string line;
   ifstream myfile ("test.txt");
   if (myfile.is_open())
   {
      if(getline (myfile,line)){
         cout << line << endl;
         myfile.close();
      }  
      else cout << "Unable to read file" << endl;
   }
   else cout << "Unable to open file" << endl; 

   return 0;
}