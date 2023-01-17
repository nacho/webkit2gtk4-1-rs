#!/bin/bash
set -x -e

# change int to uint to prevent overflow
xmlstarlet ed -L \
	-u '//_:constant[@name="DOM_NODE_FILTER_SHOW_ALL"]/_:type[@name="gint"]/@c:type' -v "guint" \
	-u '//_:constant[@name="DOM_NODE_FILTER_SHOW_ALL"]/_:type[@name="gint"]/@name' -v "guint" \
	WebKit2WebExtension-4.1.gir

xmlstarlet tr JavaScriptCore-4.1.xsl JavaScriptCore-4.1.gir | xmlstarlet fo >JavaScriptCore-4.1.gir.tmp
mv JavaScriptCore-4.1.gir.tmp JavaScriptCore-4.1.gir

# fill in types from JavaScriptCore
xmlstarlet ed -L \
	-i '///_:type[not(@name) and @c:type="JSGlobalContextRef"]' -t 'attr' -n 'name' -v "JavaScriptCore.GlobalContextRef" \
	-i '///_:type[not(@name) and @c:type="JSValueRef"]' -t 'attr' -n 'name' -v "JavaScriptCore.ValueRef" \
	WebKit2WebExtension-4.1.gir WebKit2-4.1.gir

xmlstarlet ed -L \
	-u '//_:constant[@name="DOM_NODE_FILTER_SHOW_ALL"]/_:type/@name' -v "guint" \
	-u '//_:constant[@name="DOM_NODE_FILTER_SHOW_ALL"]/_:type/@c:type' -v "guint" \
	WebKit2WebExtension-4.1.gir
