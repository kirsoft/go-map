package mymap

// int mapWrite (int numRecords);
import "C"

import (	
	//"unsafe"

	"fmt"
	"log"
	"time"
	"strconv"
)

/**
 * @brief      Calculate and print time of write and read records to Map in Go and C++
 * @param      numRecords  The number of records
 */
func Run(numRecords int) {

	// Go lang HashMap test
	fmt.Println("Run Go map test")
	start1 := time.Now()

	m := make(map[string]string)

	// Write to map
	for i := 0; i < numRecords; i++ {
		key := "string-"+strconv.Itoa(i)
		m[key] = key
	}
	log.Printf("Map took %s to write %d records", time.Since(start1), numRecords)
	start2 := time.Now()

	// Read from map
	for i := 0; i < numRecords; i++ {
		key := "string-"+strconv.Itoa(i)
		value := m[key]
		if(value == key) { }
	}
    log.Printf("Map took %s to read  %d records", time.Since(start2), numRecords)
    log.Printf("Total    %s to wite and read %d records", time.Since(start1), numRecords)

    // C++ unorderd map test
	fmt.Println("Run C++ map test")
	start3 := time.Now()

	C.mapWrite(C.int(numRecords));
    log.Printf("Map C++ took %s to write and read %d records", time.Since(start3), numRecords)
}
