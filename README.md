# Stellar Spam Filter DApp

**Stellar Spam Filter DApp** - Blockchain-Based Decentralized Email Spam Detection System

## Project Description

Stellar Spam Filter DApp is a decentralized smart contract application built on the Stellar blockchain using Soroban SDK. The system is designed to help users automatically classify and manage spam emails directly on-chain without relying on centralized email filtering services.

This decentralized application stores email data securely on the blockchain and performs simple spam detection using predefined spam keywords. Each email is categorized as either spam or non-spam, allowing users to retrieve filtered spam emails transparently and securely.

By leveraging Stellar's fast and low-cost blockchain infrastructure, the application provides a trustless and immutable spam management system that ensures transparency, persistence, and decentralized data ownership.

---

## Project Vision

Our vision is to create a decentralized communication security system that empowers users with:

- **Decentralized Email Protection**  
  Removing dependence on centralized spam filtering providers

- **Transparent Spam Detection**  
  Open and verifiable spam classification logic stored on blockchain

- **Data Ownership**  
  Giving users complete control over their email metadata and filtering system

- **Immutable Records**  
  Ensuring email activity and spam classifications cannot be manipulated

- **Blockchain-Based Security**  
  Protecting stored data through Stellar blockchain infrastructure

We believe future communication systems should be transparent, decentralized, and user-controlled.

---

## Key Features

### 1. **Email Storage**

- Store emails directly on-chain
- Save sender, subject, and content
- Automatically generate unique email IDs
- Persistent blockchain-based storage

### 2. **Spam Detection System**

- Automatic spam classification
- Keyword-based spam filtering
- Detect suspicious email content instantly
- Transparent spam verification logic

### 3. **Spam Email Retrieval**

- Retrieve only spam emails
- Efficient filtering mechanism
- Simplified frontend integration
- Fast access to suspicious messages

### 4. **Email Deletion**

- Remove emails using unique IDs
- Immediate storage update
- Efficient blockchain data management

### 5. **Stellar Blockchain Integration**

- Built using Soroban Smart Contracts
- Powered by Stellar Network
- Low transaction costs
- Fast and scalable architecture

---

## Spam Detection Logic

The current spam filter uses simple keyword-based detection.

Examples of spam keywords:

- "gratis"
- "hadiah"
- "klik"
- "promo"
- "uang"
- "menang"

If an email subject or content contains one of these keywords, the email will automatically be marked as spam.

---

## Contract Details

- Contract Address:  
  `CBLU4IUASQ4WUMOXBFLZRSBBLILGOH33GS4LUPKFBCCCMJCDQNMF7G2M`

---

## Future Scope

### Short-Term Improvements

1. **Advanced Spam Detection**
   - Smarter filtering algorithms
   - Regex pattern detection
   - Dynamic spam keyword updates

2. **Email Categories**
   - Promotions
   - Social
   - Important
   - Personal

3. **Frontend Dashboard**
   - Web-based inbox interface
   - Spam management dashboard
   - Real-time blockchain synchronization

4. **Search Functionality**
   - Search emails by sender or subject
   - Spam filtering options

---

### Medium-Term Development

5. **AI-Based Spam Classification**
   - Machine learning spam prediction
   - NLP-based email analysis
   - Adaptive spam learning system

6. **User Authentication**
   - Wallet-based login
   - Multi-user support
   - User-specific inbox storage

7. **Notification System**
   - Alerts for suspicious emails
   - Real-time spam warnings

8. **Attachment Scanning**
   - Detect suspicious file attachments
   - Malware indication system

---

### Long-Term Vision

9. **Cross-Chain Email Security**
   - Multi-blockchain spam protection

10. **Decentralized Messaging Platform**
   - Fully decentralized communication ecosystem

11. **AI Cybersecurity Integration**
   - Intelligent blockchain cybersecurity assistant

12. **Privacy Protection**
   - End-to-end encrypted email storage
   - Zero-knowledge proof integration

13. **DAO Governance**
   - Community-controlled spam rules
   - Voting for spam detection improvements

---

## Technical Requirements

- Soroban SDK
- Rust Programming Language
- Stellar Blockchain Network

---

## Smart Contract Functions

### `add_email()`

Store a new email and automatically classify it as spam or non-spam.

### `get_emails()`

Retrieve all stored emails.

### `get_spam_emails()`

Retrieve only emails marked as spam.

### `delete_email()`

Delete an email using its unique ID.

---

## Technologies Used

- Rust
- Soroban SDK
- Stellar Blockchain
- Smart Contracts

---

# Stellar Spam Filter DApp

Securing Digital Communication Through Blockchain Technology