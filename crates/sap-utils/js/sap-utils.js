export function format(str) {
	const div = document.createElement(`div`);
	div.innerHTML = str.trim();

	return format_raw(div, 0).innerHTML;
}

function format_raw(node, level) {
	var indentBefore = new Array(level++ + 1).join('  '),
		indentAfter = new Array(level - 1).join('  '),
		textNode;

	for (var i = 0; i < node.children.length; i++) {
		textNode = document.createTextNode('\n' + indentBefore);
		node.insertBefore(textNode, node.children[i]);

		format_raw(node.children[i], level);

		if (node.lastElementChild == node.children[i]) {
			textNode = document.createTextNode('\n' + indentAfter);
			node.appendChild(textNode);
		}
	}

	return node;
}

export function wait_promise(ms) {
	return new Promise((resolve) => {
		let wait = setTimeout(() => {
			clearTimeout(wait);
			resolve();
		}, ms)
	})
}

export function until_mutation(element, action, timeout) {
	return new Promise((resolve, reject) => {
		const observerOptions = {
			childList: true,
			attributes: true,
			subtree: true,
			characterData: true,
		};

		const observer = new MutationObserver(() => { resolve() });
		observer.observe(element, observerOptions);
		action()
		if (timeout) {
			let wait = setTimeout(() => {
				clearTimeout(wait);
				reject(`No change observed within the allotted time: ${timeout}ms.`);
			}, timeout)
		}
	});
}