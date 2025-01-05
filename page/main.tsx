import { Route } from "wouter";
import Layout from "./components/Layout";
import Series from "./components/pages/Series";
import Top from "./components/pages/Top";

export function Main() {
	return (
		<Layout>
			<Route path="/">
				<Top />
			</Route>
			<Route path="/series/:id">{({ id }) => <Series id={id} />}</Route>
		</Layout>
	);
}
