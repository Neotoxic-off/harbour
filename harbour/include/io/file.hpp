#pragma once

#include <iostream>
#include <fstream>
#include <string>
#include <filesystem>

class File
{
    public:
        std::string path;
        bool exists;

        File();
        ~File();

        bool Exists();
        bool Create();
        bool Delete();
        void Write(const std::string content);
        std::string Read();
};
