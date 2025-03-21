#pragma once

#include <iostream>
#include <filesystem>

#include "spdlog/spdlog.h"

#include "io/directory.hpp"
#include "io/file.hpp"

#include "arguments.hpp"

class Core
{
    private:
        Arguments arguments;
        Directory config_directory;

        int argc;
        char** argv;

        std::string GetConfigDirectory();
 
    public:
        Core(int argc, char** argv);
        ~Core();
        
        void Initialize();
        void Parse();
};
