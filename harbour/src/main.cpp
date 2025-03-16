#include "core.hpp"

int main(int argc, char** argv)
{
    Core core = Core(argc, argv);

    core.Parse();

    return 0; 
}
