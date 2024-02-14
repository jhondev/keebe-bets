declare global {
	interface BigInt {
		toJSON(): string;
	}
}
BigInt.prototype.toJSON = function () {
	return this.toString();
};

export const genUUID = () => {
	return BigInt(Math.floor(Date.now() * Math.random()));
};

export const formatAmount = (amount: bigint | number) => BigInt(amount) / BigInt(100000000);
