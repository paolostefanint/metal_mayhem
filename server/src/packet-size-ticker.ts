export const createPacketSizeTicker = function () {
    let dataAtLastMeasurement = 0;
    let currentData = 0;

    let sumPacketSize = 0;
    let packagesFromLastMeasurement = 0;

    setInterval(() => {
        let dataInLastSecond = currentData - dataAtLastMeasurement;
        dataAtLastMeasurement = currentData;

        let avgPacketSize =
            packagesFromLastMeasurement > 0
                ? sumPacketSize / packagesFromLastMeasurement
                : 0;
        packagesFromLastMeasurement = 0;
        sumPacketSize = 0;

        // console.log(`core data rate / avg package size: ${dataInLastSecond} B/s / ${avgPacketSize} B`);
    }, 2000);

    return function (data: ArrayBuffer) {
        currentData += data.byteLength;
        sumPacketSize += data.byteLength;
        packagesFromLastMeasurement++;
    };
};
