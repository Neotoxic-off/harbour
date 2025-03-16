#pragma once

#include <argparse/argparse.hpp>

#include "settings.hpp"

class Arguments
{
    public:
        Arguments();
        ~Arguments();
    
        void Parse(int argc, char** argv);
};
