#include <string>
#include <iostream>
#include <unordered_map>
 
extern "C" int mapWrite (int numRecords)
{	
    std::unordered_map<std::string, std::string> m;

    for(int i=0; i < numRecords; i++) {
    	auto key = "string-" + std::to_string(i);
    	m.insert({key, key});
	}

	for(int i=0; i < numRecords; i++) {
    	auto key = "string-" + std::to_string(i);
    	auto value = m[key];
    	if(key == value);
	}

	return 0;
}
