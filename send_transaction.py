import hashlib
import base64
import random
import json
from sawtooth_sdk.protobuf.transaction_pb2 import TransactionHeader, Transaction
from sawtooth_sdk.client import Client
from sawtooth_sdk.signing import LocalSigner
from sawtooth_sdk.crypto import CryptoFactory

# Validator URL
VALIDATOR_URL = "http://localhost:8800"

# Define the family name and version
FAMILY_NAME = "intkey"
FAMILY_VERSION = "1.0"

# Load the signer (keypair) for the client
def load_signer(client_id):
    with open(f'{client_id}_private.pem', 'rb') as f:
        private_key = f.read()
    signer = LocalSigner(private_key)
    return signer

# Create the transaction payload for IntKey family
def create_payload(key, value):
    payload = {
        "action": "SET",
        "key": key,
        "value": value
    }
    return json.dumps(payload).encode('utf-8')

# Create the transaction
def send_transaction(sender_signer, key, value):
    # Generate payload
    payload = create_payload(key, value)
    payload_sha512 = hashlib.sha512(payload).hexdigest()

    # Create transaction header
    header = TransactionHeader(
        family_name=FAMILY_NAME,
        family_version=FAMILY_VERSION,
        signer=sender_signer.get_public_key().as_hex(),
        payload_sha512=payload_sha512
    )

    # Create the transaction
    transaction = Transaction(
        header=header,
        header_signature=base64.b64encode(random.randbytes(32)).decode('utf-8'),
        payload=payload
    )

    # Send transaction to validator
    client = Client(VALIDATOR_URL)
    client.send_transaction(transaction)
    print(f"Transaction sent: {key} -> {value}")

# Main function to send updates from client1 to client100
def main():
    # Client 1 sends updates to client 100
    for i in range(1, 100):
        sender_signer = load_signer(f'client{i}')
        recipient_key = f'key_{i}'
        recipient_value = random.randint(1, 100000)  # Random value for the key
        send_transaction(sender_signer, recipient_key, recipient_value)

if __name__ == "__main__":
    main()
