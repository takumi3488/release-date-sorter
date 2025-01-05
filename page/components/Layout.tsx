import Footer from "./templates/Footer";
import Header from "./templates/Header";

export default function Layout({ children }: { children: React.ReactNode }) {
	return (
		<div>
			<Header />
			<main>{children}</main>
			<Footer />
		</div>
	);
}
