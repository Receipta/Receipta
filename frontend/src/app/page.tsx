export default function Home() {
  return (
    <main className="min-h-screen flex flex-col items-center justify-center p-8">
      <h1 className="text-4xl font-bold mb-4">Receipta</h1>
      <p className="text-lg text-gray-600 mb-8">
        Tamper-proof payment verification on Stellar
      </p>
      <a
        href="/verify"
        className="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
      >
        Verify a Receipt
      </a>
    </main>
  );
}
