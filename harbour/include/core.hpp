#pragma once

#include <iostream>

#include "spdlog/spdlog.h"

#include "arguments.hpp"

class Core
{
    private:
        Arguments arguments;

        int argc;
        char** argv;
        
        void Initialize();

    public:
        Core(int argc, char** argv);
        ~Core();

        void Parse();
};
