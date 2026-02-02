public final class NeurochemSleepGate {

    private final long hashSalt;

    public NeurochemSleepGate(long hashSalt) {
        this.hashSalt = hashSalt;
    }

    public static final class StagePosteriors {
        public final double pWake, pN1, pN2, pN3, pREM;
        public StagePosteriors(double pWake, double pN1, double pN2, double pN3, double pREM) {
            this.pWake = pWake;
            this.pN1 = pN1;
            this.pN2 = pN2;
            this.pN3 = pN3;
            this.pREM = pREM;
        }
    }

    public static final class NeurochemInputs {
        public final double lfHfNorm;      // 0..1
        public final double rmssdNorm;     // 0..1
        public final double stressNorm;    // 0..1
        public final double crpProxy;      // 0..1 (optional)
        public final double il6Proxy;      // 0..1 (optional)
        public NeurochemInputs(double lfHfNorm, double rmssdNorm,
                               double stressNorm, double crpProxy, double il6Proxy) {
            this.lfHfNorm = lfHfNorm;
            this.rmssdNorm = rmssdNorm;
            this.stressNorm = stressNorm;
            this.crpProxy = crpProxy;
            this.il6Proxy = il6Proxy;
        }
    }

    public static final class GateResult {
        public final double dN2N3;
        public final double uQuestion;
        public final double uEntropy;
        public final double uCombined;
        public final double gSafe;
        public final double neurochemLoad;
        public final double recoveryIndex;
        public final String hexCommit;
        public GateResult(double dN2N3,
                          double uQuestion,
                          double uEntropy,
                          double uCombined,
                          double gSafe,
                          double neurochemLoad,
                          double recoveryIndex,
                          String hexCommit) {
            this.dN2N3 = dN2N3;
            this.uQuestion = uQuestion;
            this.uEntropy = uEntropy;
            this.uCombined = uCombined;
            this.gSafe = gSafe;
            this.neurochemLoad = neurochemLoad;
            this.recoveryIndex = recoveryIndex;
            this.hexCommit = hexCommit;
        }
    }

    public GateResult evaluate(StagePosteriors p,
                               double slowWaveIndex,
                               NeurochemInputs nc) {

        double[] post = renormalize(p);
        double pW = post[0], pN1 = post[1], pN2 = post[2], pN3 = post[3], pREM = post[4];

        double dBand = 0.4 * pN2 + 1.2 * pN3 + 0.4 * slowWaveIndex;
        double dN2N3 = clamp01(dBand);

        double uQuestion = computeUMaxGap(post);
        double uEntropy = computeUEntropy(post);
        double uCombined = 0.5 * (uQuestion + uEntropy);

        double gSafe = clamp01(dN2N3 * (1.0 - uCombined));

        double neurochemLoad = computeNeurochemLoad(dN2N3, uCombined, nc);
        double recoveryIndex = computeRecoveryIndex(dN2N3, nc);

        double[] features = new double[] {
                dN2N3, uCombined, gSafe,
                nc.lfHfNorm, nc.rmssdNorm, nc.stressNorm,
                nc.crpProxy, nc.il6Proxy
        };
        String hex = hashToHex64(quantizeToBytes(features));

        return new GateResult(dN2N3, uQuestion, uEntropy, uCombined,
                              gSafe, neurochemLoad, recoveryIndex, hex);
    }

    private static double clamp01(double x) {
        if (!Double.isFinite(x)) return 0.0;
        if (x < 0.0) return 0.0;
        if (x > 1.0) return 1.0;
        return x;
    }

    private static double[] renormalize(StagePosteriors p) {
        double[] arr = new double[] {
                clamp01(p.pWake), clamp01(p.pN1),
                clamp01(p.pN2), clamp01(p.pN3),
                clamp01(p.pREM)
        };
        double sum = 0.0;
        for (double v : arr) sum += v;
        if (sum <= 0.0) sum = 1e-9;
        for (int i = 0; i < arr.length; i++) {
            arr[i] = arr[i] / sum;
        }
        return arr;
    }

    private static double computeUMaxGap(double[] post) {
        double max = 0.0;
        for (double v : post) {
            if (v > max) max = v;
        }
        return clamp01(1.0 - max);
    }

    private static double computeUEntropy(double[] post) {
        double h = 0.0;
        for (double v : post) {
            if (v > 0.0) {
                h -= v * Math.log(v);
            }
        }
        double hMax = Math.log(5.0);
        if (hMax <= 0.0) return 0.0;
        return clamp01(h / hMax);
    }

    private static double computeNeurochemLoad(double dN2N3,
                                               double uCombined,
                                               NeurochemInputs nc) {
        double invDepth = 1.0 - dN2N3;
        double stress = clamp01(nc.stressNorm);
        double lfHf = clamp01(nc.lfHfNorm);
        double crp = clamp01(nc.crpProxy);
        double il6 = clamp01(nc.il6Proxy);

        double raw = 0.30 * stress
                   + 0.20 * lfHf
                   + 0.20 * invDepth
                   + 0.15 * uCombined
                   + 0.075 * crp
                   + 0.075 * il6;
        return clamp01(raw);
    }

    private static double computeRecoveryIndex(double dN2N3,
                                               NeurochemInputs nc) {
        double rmssd = clamp01(nc.rmssdNorm);
        double stressLow = 1.0 - clamp01(nc.stressNorm);
        double raw = 0.40 * rmssd
                   + 0.40 * dN2N3
                   + 0.20 * stressLow;
        return clamp01(raw);
    }

    private byte[] quantizeToBytes(double[] features) {
        int n = features.length;
        byte[] out = new byte[n];
        for (int i = 0; i < n; i++) {
            double f = clamp01(features[i]);
            int q = (int) Math.round(f * 255.0);
            if (q < 0) q = 0;
            if (q > 255) q = 255;
            out[i] = (byte) (q & 0xFF);
        }
        return out;
    }

    private String hashToHex64(byte[] data) {
        long h = hashSalt;
        for (byte b : data) {
            int v = b & 0xFF;
            h ^= (long) v;
            h *= 0x9E3779B185EBCA87L; // mix
            h = Long.rotateLeft(h, 27);
        }
        long hi = h * 0xA5A5A5A585EBCA6BL;
        long lo = h * 0x27D4EB2FC2B2AE35L;
        return toHex32(hi) + toHex32(lo);
    }

    private static String toHex32(long x) {
        long v = x & 0xFFFFFFFFL;
        String s = Long.toHexString(v);
        if (s.length() < 8) {
            StringBuilder sb = new StringBuilder(8);
            for (int i = s.length(); i < 8; i++) sb.append('0');
            sb.append(s);
            return sb.toString();
        }
        return s.substring(s.length() - 8);
    }
}
