
    // SOLIDITY VERSION OF FUNCTIONS  
    // SPDX-License-Identifier: MIT
    pragma solidity ^0.8.17;

    contract DataStorage {

        // These are state variables. They are stored on the blockchain and represent the "state" of this contract.
        // Changes to these variables are recorded on the blockchain.
        string public myString = "Hello, World!";
        uint256 public myUint = 12345;

        // This function returns the bytes representation of the string `myString`.
        // It's marked as `view` because it doesn't modify the contract state.
        // The returned data is placed in memory, which is a temporary data storage area.
        function getStringBytes() external view returns (bytes memory) {
            return bytes(myString);  // Converts the string to bytes and returns it.
        }

        // This function returns the bytes32 representation of the uint `myUint`.
        // Like above, it's a `view` function, and the returned data is placed in memory.
        function getUintBytes() external view returns (bytes32) {
            return bytes32(myUint);  // Converts the uint to bytes32 and returns it.
        }

        // This function returns a hexadecimal string representation of `myString`.
        // It's a `view` function as it doesn't change the contract state.
        // It utilizes memory for temporary data storage during its execution.
        function getStringHex() external view returns (string memory) {
            bytes memory strBytes = bytes(myString);  // Converts the string to bytes.
            bytes memory hexBytes = new bytes(strBytes.length * 2);  // Allocates memory for the hex representation.
            bytes16 hexAlphabet = 0x30313233343536373839616263646566;  // Hexadecimal characters as bytes.

            // Loop through each character of `strBytes`, converting it to hex and storing it in `hexBytes`.
            for (uint i = 0; i < strBytes.length; i++) {
                hexBytes[i*2] = hexAlphabet[uint(uint8(strBytes[i]) >> 4)];
                hexBytes[i*2 + 1] = hexAlphabet[uint(uint8(strBytes[i]) & 0x0f)];
            }

            return string(hexBytes);  // Converts the bytes to string and returns it.
        }

        // This function returns a hexadecimal string representation of `myUint`.
        // It calls a helper function `uintToHexStr` to perform the conversion.
        function getUintHex() external view returns (string memory) {
            return uintToHexStr(myUint);  // Calls the helper function with `myUint` as argument.
        }

        // This helper function converts a uint to a hexadecimal string.
        // It's marked as `pure` since it doesn't access or modify the contract state.
        // It uses a do-while loop to build the hex string, utilizing memory for temporary data storage.
        function uintToHexStr(uint256 inputValue) internal pure returns (string memory) {
            bytes memory buffer = new bytes(32);  // Temporary buffer to hold the result.
            uint256 index = 32;  // Index for tracking the position in `buffer`.
        
            // Loop until `inputValue` is zero, building the hex string in reverse.
            do {
                buffer[--index] = bytes1((inputValue % 16 < 10 ? 48 : 87) + uint8(inputValue % 16));
                inputValue /= 16;
            } while (inputValue != 0);
        
            // Copy the relevant part of `buffer` to `result` and return it.
            bytes memory result = new bytes(32 - index);
            for (uint i = 0; i < result.length; i++) {
                result[i] = buffer[index++];
            }
            return string(result);
        }
    }
