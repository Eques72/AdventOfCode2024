#include <regex>
#include <iostream>
#include <fstream>

void read_input(std::vector<std::string> &input) {
    std::ifstream file("input_task_3.txt");
    if (file.is_open())
    {
        std::string str;
        while (std::getline(file, str))
        {
            input.push_back(str);
        }
    }
}

std::vector<std::string> use_regex(const std::vector<std::string> &input, const std::regex reg) {
    std::vector<std::string> matches;
    for(const auto& line : input){
        for (std::sregex_iterator it = std::sregex_iterator(line.begin(), line.end(), reg);
            it != std::sregex_iterator(); it++) {
            std::smatch match;
            match = *it;
            matches.push_back(match.str(0));
        }
    }
    return matches;
}



enum State{
    none,
    afterDo,
    afterDont
};

int part = 2;

int main()
{
    std::vector<std::string> input;
    read_input(input);
if(part == 1){
    const std::regex mulReg("(mul\\([1-9]\\d*,[1-9]\\d*\\))");

    std::vector<std::string> matches = use_regex(input, mulReg);

    std::vector<std::vector<int>> pairs(matches.size());
    const std::regex numReg("(\\d*)");
    for(size_t i = 0;i < matches.size(); i++){
    for (std::sregex_iterator it = std::sregex_iterator(matches[i].begin(), matches[i].end(), numReg);
        it != std::sregex_iterator(); it++)
        {
            std::smatch match;
            match = *it;

            if(match.str(0) != ""){
            std::cout << "Matched: " << match.str(0) << std::endl;
            pairs[i].push_back(stoi(match.str(0)));
            }
        }
    }

    long long res = 0;
    for(auto pair : pairs)
    {
    res += pair[0]*pair[1];
    }
    std::cout << "res: " << res << std::endl;
}
if(part == 2){
    const std::regex mulReg("(mul\\([1-9]\\d*,[1-9]\\d*\\)|do\\(\\)|don't\\(\\))");
    std::vector<std::string> matches = use_regex(input, mulReg);

    for(auto v : matches)
        std::cout << v << std::endl;

    State state = afterDo;
    std::vector<std::string> clearMatches;
    for(size_t i = 0;i < matches.size(); i++){
        if(matches[i] == "do()")
            state = afterDo;
        else if(matches[i] == "don't()")
            state = afterDont;
        else
        {
            if(state == afterDo)
                clearMatches.push_back(matches[i]);
           // state = none;
        }
    }

    for(auto v : clearMatches)
        std::cout << v << std::endl;

    std::vector<std::vector<int>> pairs(clearMatches.size());
    const std::regex numReg("(\\d*)");
    for(size_t i = 0;i < clearMatches.size(); i++){
    for (std::sregex_iterator it = std::sregex_iterator(clearMatches[i].begin(), clearMatches[i].end(), numReg);
        it != std::sregex_iterator(); it++)
        {
            std::smatch match;
            match = *it;

            if(match.str(0) != ""){
            std::cout << "Matched: " << match.str(0) << std::endl;
            pairs[i].push_back(stoi(match.str(0)));
            }
        }
    }

//    vec.erase(vec.begin() + index);
    long long res = 0;
    for(auto pair : pairs)
    {
    res += pair[0]*pair[1];
    }
    std::cout << "res: " << res << std::endl;
}
    return 0;
}
