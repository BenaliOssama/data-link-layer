# data-link-layer

A learning implementation of **data link layer protocols** in Rust, built on top of an unreliable physical channel emulator.

## Features
- **Stop-and-Wait (PAR)** with timeout and sequence numbers
- **Go-Back-N Sliding Window**
- **Selective Repeat Sliding Window**
- **CRC-16 checksum** for error detection
- Works with a **physical layer emulator** simulating packet loss and corruption

## Getting Started

### Prerequisites
- Rust 1.70+ (or latest stable)
- Cargo package manager
- Physical layer emulator provided in course materials

### Running
1. Clone the repo:
   ```bash
   git clone https://github.com/yourusername/data-link-layer.git
   cd data-link-layer

### resources
https://www.cs.fsu.edu/~xyuan/cen5515/
