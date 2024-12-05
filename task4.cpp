#include <iostream>
#include <fstream>
#include <vector>

void read_input(std::vector<std::string> &input) {
    std::ifstream file("input_task_4.txt");
    if (file.is_open())
    {
        std::string str;
        while (std::getline(file, str))
        {
            input.push_back(str);
        }
    }
}

int part = 2;

//XMAS
int main()
{
    std::vector<std::string> input;
    read_input(input);
if(part == 1){
    int foundXmas = 0;

    for(int i = 0; i < input.size(); i++){
    for(int j = 0; j < input[i].size(); j++) //width
    {
    //     j
    //[][][a ][][]
    //[][][m ][][]
    //[][][X][][]i
    //[][][ ][][]
    //[][][ ][][]
        if(input[i][j] == 'X')
        {
            std::cout << "X " << i << " : "<< j << std::endl;
            if(i-3 >= 0 && j-3 >= 0)
            {
                if(input[i-1][j-1] == 'M')
                    if(input[i-2][j-2] == 'A')
                        if(input[i-3][j-3] == 'S'){
                            foundXmas++;}
            }
            if(i-3 >= 0)
                if(input[i-1][j] == 'M')
                    if(input[i-2][j] == 'A')
                        if(input[i-3][j] == 'S'){
                            foundXmas++;}
            if(i-3 >= 0 && j+3 < input[i].size())
                if(input[i-1][j+1] == 'M')
                    if(input[i-2][j+2] == 'A')
                        if(input[i-3][j+3] == 'S'){
                            foundXmas++;}
            if(j-3 >= 0)
                if(input[i][j-1] == 'M')
                    if(input[i][j-2] == 'A')
                        if(input[i][j-3] == 'S'){
                            foundXmas++;}
            if(j+3 < input[i].size())
                if(input[i][j+1] == 'M')
                    if(input[i][j+2] == 'A')
                        if(input[i][j+3] == 'S'){
                            foundXmas++;}
            if(i+3 < input.size() && j-3 >= 0)
                if(input[i+1][j-1] == 'M')
                    if(input[i+2][j-2] == 'A')
                        if(input[i+3][j-3] == 'S'){
                            foundXmas++;}
            if(i+3 < input.size())
                if(input[i+1][j] == 'M')
                    if(input[i+2][j] == 'A')
                        if(input[i+3][j] == 'S'){
                            foundXmas++;}
            if(i+3 < input.size() && j+3 < input[i].size())
                if(input[i+1][j+1] == 'M')
                    if(input[i+2][j+2] == 'A')
                        if(input[i+3][j+3] == 'S'){
                            foundXmas++;}
        }

    }
    }
    std::cout << "Total XMAS: " << std::endl;
    std::cout << foundXmas;
}
if(part == 2){
    int foundXmas = 0;

    for(int i = 1; i < input.size()-1; i++){
    for(int j = 1; j < input[i].size()-1; j++)
    {
        if(input[i][j] == 'A')
        {
            bool backslash = false;
            bool forwardslash = false;
            if(input[i-1][j-1] == 'M')
                if(input[i+1][j+1] == 'S')
                        backslash = true;
             if(input[i+1][j+1] == 'M')
                if(input[i-1][j-1] == 'S')
                    backslash = true;

            if(input[i+1][j-1] == 'M')
                if(input[i-1][j+1] == 'S')
                    forwardslash = true;
             if(input[i-1][j+1] == 'M')
                if(input[i+1][j-1] == 'S')
                    forwardslash = true;
            if(forwardslash && backslash)
                foundXmas++;
        }
    }
    }
    std::cout << "Total MAX but in an X: " << std::endl;
    std::cout << foundXmas;
}
    return 0;
}
