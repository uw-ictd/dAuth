import hashlib
import random
import requests
import base64
import yaml
import json
import cbor # Go ahead and remove the dependency on this with protobuf serialization format.

from sawtooth_sdk.protobuf.transaction_pb2 import TransactionHeader
from sawtooth_sdk.protobuf.transaction_pb2 import Transaction
from sawtooth_sdk.protobuf.batch_pb2 import BatchHeader
from sawtooth_sdk.protobuf.batch_pb2 import BatchList
from sawtooth_sdk.protobuf.batch_pb2 import Batch

from sawtooth_ccellular.structures.structures_pb2 import DatabaseInstruction

from sawtooth_ccellular.client.constants import BINARY_NAME, BINARY_VERSION

from sawtooth_signing import create_context
from sawtooth_signing import CryptoFactory
from sawtooth_signing import ParseError
from sawtooth_signing.secp256k1 import Secp256k1PrivateKey


def _sha512(data):
    return hashlib.sha512(data).hexdigest()


class CCellularClient:
    def __init__(self, url, keyfile=None):
        self.url = url
        if keyfile is not None:
            try:
                with open(keyfile) as fd:
                    private_key_str = fd.read().strip()
                    fd.close()
            except OSError as err:
                raise Exception('Failed to read private key at {}'.format(err))

            try:
                private_key = Secp256k1PrivateKey.from_hex(private_key_str)
            except ParseError as e:
                raise Exception('Unable to load the private key correctly {}'.format(str(e)))

            self._signer = CryptoFactory(create_context('secp256k1')).new_signer(private_key)
            print(self._signer)

    def set(self, imsi, value):
        # print("Trying to set {} to {}".format(imsi, value))
        return self._send_transaction('set', imsi, value)

    def get(self, imsi):
        address = self._get_address(imsi)
        print("Looking for {k} at {a}".format(k=imsi, a=address))
        result = self._send_request("state/{}".format(address), name=imsi,)
        try:
            json_result = json.loads(result)
            data_response = json_result['data']
            b64data = yaml.safe_load(data_response)
            b64decoded = base64.b64decode(b64data)
            cbor_decoded = cbor.loads(b64decoded)
            instruction = DatabaseInstruction()
            instruction.ParseFromString(cbor_decoded[imsi])
            return cbor_decoded[imsi]
        except BaseException as e:
            print("Received a base exception. " + e)
            return None


    # Private methods used by the client
    @staticmethod
    def _get_prefix():
        return _sha512(BINARY_NAME.encode('utf-8'))[0:6]

    def _get_address(self, imsi):
        prefix = self._get_prefix()
        address = _sha512(imsi.encode('utf-8'))[64:]
        return prefix + address

    def _send_request(self, suffix, data=None, content_type=None, name=None):
        if self.url.startswith("http://"):
            url = "{}/{}".format(self.url, suffix)
        else:
            url = "http://{}/{}".format(self.url, suffix)
        headers = {}

        if content_type is not None:
            headers['Content-Type'] = content_type

        try:
            if data is not None:
                result = requests.post(url, headers=headers, data=data)
            else:
                result = requests.get(url, headers=headers)

            if result.status_code == 404:
                raise Exception("No such IMSI Exists: {}".format(name))

            if not result.ok:
                raise Exception("Error {}: {}".format(result.status_code, result.reason))
        except requests.ConnectionError as err:
            raise Exception("Failed to connect to the REST API services : {}".format(err))
        except BaseException as err:
            raise Exception("Failed {}".format(err))
        return result.text

    def _send_transaction(self, operation, imsi, value):
        payload = cbor.dumps({'verb': operation, 'imsi': imsi, 'value': value})
        # Construct the address
        address = self._get_address(imsi)
        header = TransactionHeader(
            signer_public_key=self._signer.get_public_key().as_hex(),
            family_name=BINARY_NAME,
            family_version=BINARY_VERSION,
            inputs=[address],
            outputs=[address],
            dependencies=[],
            payload_sha512=_sha512(payload),
            batcher_public_key=self._signer.get_public_key().as_hex(),
            nonce=hex(random.randint(0, 2 ** 64))
        ).SerializeToString()

        # print("Header : {}".format(header))

        signature = self._signer.sign(header)
        # print("Signature: {}".format(signature))

        transaction = Transaction(
            header=header,
            payload=payload,
            header_signature=signature
        )

        # print("Transaction: {}".format(transaction))

        batch_list = self._create_batch_list([transaction])

        return self._send_request(
            "batches", batch_list.SerializeToString(),
            'application/octet-stream',
        )

    def _create_batch_list(self, transactions):
        transaction_signatures = [t.header_signature for t in transactions]

        header = BatchHeader(
            signer_public_key=self._signer.get_public_key().as_hex(),
            transaction_ids=transaction_signatures
        ).SerializeToString()

        signature = self._signer.sign(header)

        batch = Batch(
            header=header,
            transactions=transactions,
            header_signature=signature)
        return BatchList(batches=[batch])
