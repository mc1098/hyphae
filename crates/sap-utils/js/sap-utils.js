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

export function get_value(element) {
	return element.value
}

export function set_value(element, value) {
	if (element.value !== undefined) {
		element.value = value;
		return true;
	} else {
		return false;
	}
}