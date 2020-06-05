#include <cstdio>
#include <string>
#include <cstdlib>

//Open the file /dev/cpu/$(cpu)/msr and return the value of the desired register
//Return 0 if there was an error in reading the file
unsigned long long read_from_cpu(int cpu, unsigned long reg){
    std::string path_to_file = std::string("/dev/cpu/") + std::string(std::to_string(cpu)) + std::string("/msr");
    FILE * f = fopen(path_to_file.c_str(), "r");
    if(f == NULL){
        return 0;
    }
    //seeking to the position we need
    fseek(f, reg, SEEK_SET);

    //reading 8 bytes and returning them
    char bytes[8];
    fread(bytes, 1, 8, f);
    //returning the read value as an unsigned long long
    return * (unsigned long long *) bytes;
}

int main(int argc, char * argv[]){
    //parsing argument into register value
    unsigned long reg = std::stoul(argv[1], nullptr, 16);
    if(reg == 0){
        printf("Invalid MSR identifier inputted. Please enter a hexadecimal integer.\n");
        return 0;
    }

    //reading the requested value from all cpus
    for(int cpu = 0; cpu < 16 ; cpu++){
        unsigned long long value = read_from_cpu(cpu, reg);
        if(value == 0){
            printf("Problem reading from cpu %d. Exiting.\n", cpu);
            return 0;
        }
        printf("Value of MSR %lx for cpu %d: %llx\n", reg, cpu, value);
    }

    return 0;
}