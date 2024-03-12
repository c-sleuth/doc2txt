doc2txt
===
doc2txt is a tool designed to extract text from .doc files within a specified directory, this may be useful for recovering the text from corrupted .doc files

## Features

### File Handling 
Initially attempts to load files in a 1024-byte buffer. Files smaller than this buffer are logged as errors and skipped, optimizing the process for compatible file sizes.

### Signature Verification
Reads byte offset 0x200 (512) to verify the .doc file signature, ensuring that only valid .doc files are processed.

### Text Extraction 
Targets the byte stream starting from offset 0xA00, where the actual text content is located, up to the point where null bytes are encountered to stop the read operation.

### Output Management
Extracted text is written to .txt files in the specified output directory.

## Logs 
The process generates two types of log files for monitoring and troubleshooting:
- error_log.txt: Captures expected errors, such as issues with the initial file buffering, for transparency and debugging.
- successfully_recovered_log.txt: Logs all successfully processed files, providing a clear record of the operation's success.

## Usage

```bash
doc2txt -d {directory of .doc files} -o {output directory}
```

### Options

-h: Displays help information, listing all available options.
-d: Specifies the directory containing the .doc files to be processed.
-o: Defines the destination directory for the extracted text files.

## Getting Started

1. Ensure that your environment is set up to run executable files.
2. Place doc2txt executable in a convenient location.
3. Open a command-line interface and navigate to the directory containing doc2txt.
4. Use the above command examples to start extracting text from your .doc files.

## Contribution

Contributions are welcome! If you have suggestions for improvements or have identified bugs, please feel free to submit an issue or pull request on the project's GitHub page.
