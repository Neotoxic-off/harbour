#pragma once

#include <filesystem>
#include <iostream>
#include <string>
#include <vector>

class Directory
{
    public:
        std::string path;
        bool exists;

        Directory();
        ~Directory();

        bool Exists();
        bool Delete();
        bool Create();
        std::vector<std::string> List();
};
