#pragma once

#include <filesystem>
#include <iostream>
#include <string>
#include <vector>

class Directory
{
    public:
        const char* path;
        bool exists;

        Directory();
        ~Directory();

        bool Exists();
        bool Delete();
        bool Create();
        std::vector<std::string> List();
};
