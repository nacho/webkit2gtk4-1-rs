#!/bin/bash
set -x -e

# change int to uint to prevent overflow
xmlstarlet ed -L \
	-u '//_:constant[@name="DOM_NODE_FILTER_SHOW_ALL"]/_:type[@name="gint"]/@c:type' -v "guint" \
	-u '//_:constant[@name="DOM_NODE_FILTER_SHOW_ALL"]/_:type[@name="gint"]/@name' -v "guint" \
	WebKit2WebExtension-5.0.gir

xmlstarlet tr JavaScriptCore-5.0.xsl JavaScriptCore-5.0.gir | xmlstarlet fo >JavaScriptCore-5.0.gir.tmp
mv JavaScriptCore-5.0.gir.tmp JavaScriptCore-5.0.gir

# fill in types from JavaScriptCore
xmlstarlet ed -L \
	-i '///_:type[not(@name) and @c:type="JSGlobalContextRef"]' -t 'attr' -n 'name' -v "JavaScriptCore.GlobalContextRef" \
	-i '///_:type[not(@name) and @c:type="JSValueRef"]' -t 'attr' -n 'name' -v "JavaScriptCore.ValueRef" \
	WebKit2WebExtension-5.0.gir WebKit2-5.0.gir

xmlstarlet ed -L \
	-u '//_:constant[@name="DOM_NODE_FILTER_SHOW_ALL"]/_:type/@name' -v "guint" \
	-u '//_:constant[@name="DOM_NODE_FILTER_SHOW_ALL"]/_:type/@c:type' -v "guint" \
	WebKit2WebExtension-5.0.gir
