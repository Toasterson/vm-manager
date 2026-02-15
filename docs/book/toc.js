// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="part-title">Getting Started</li><li class="chapter-item expanded "><a href="getting-started/installation.html"><strong aria-hidden="true">1.</strong> Installation</a></li><li class="chapter-item expanded "><a href="getting-started/prerequisites.html"><strong aria-hidden="true">2.</strong> Prerequisites</a></li><li class="chapter-item expanded "><a href="getting-started/quick-start.html"><strong aria-hidden="true">3.</strong> Quick Start</a></li><li class="chapter-item expanded affix "><li class="part-title">Concepts</li><li class="chapter-item expanded "><a href="concepts/how-it-works.html"><strong aria-hidden="true">4.</strong> How vmctl Works</a></li><li class="chapter-item expanded "><a href="concepts/imperative-vs-declarative.html"><strong aria-hidden="true">5.</strong> Imperative vs Declarative</a></li><li class="chapter-item expanded "><a href="concepts/vm-lifecycle.html"><strong aria-hidden="true">6.</strong> VM Lifecycle</a></li><li class="chapter-item expanded "><a href="concepts/networking.html"><strong aria-hidden="true">7.</strong> Networking Modes</a></li><li class="chapter-item expanded "><a href="concepts/image-management.html"><strong aria-hidden="true">8.</strong> Image Management</a></li><li class="chapter-item expanded "><a href="concepts/cloud-init-ssh.html"><strong aria-hidden="true">9.</strong> Cloud-Init and SSH Keys</a></li><li class="chapter-item expanded affix "><li class="part-title">Tutorials</li><li class="chapter-item expanded "><a href="tutorials/imperative-vm.html"><strong aria-hidden="true">10.</strong> Creating a VM Imperatively</a></li><li class="chapter-item expanded "><a href="tutorials/declarative-workflow.html"><strong aria-hidden="true">11.</strong> Declarative Workflow with VMFile.kdl</a></li><li class="chapter-item expanded "><a href="tutorials/provisioning.html"><strong aria-hidden="true">12.</strong> Provisioning</a></li><li class="chapter-item expanded "><a href="tutorials/omnios-builder.html"><strong aria-hidden="true">13.</strong> Real-World: OmniOS Builder VM</a></li><li class="chapter-item expanded affix "><li class="part-title">VMFile.kdl Reference</li><li class="chapter-item expanded "><a href="vmfile/overview.html"><strong aria-hidden="true">14.</strong> Overview</a></li><li class="chapter-item expanded "><a href="vmfile/vm-block.html"><strong aria-hidden="true">15.</strong> VM Block</a></li><li class="chapter-item expanded "><a href="vmfile/image-sources.html"><strong aria-hidden="true">16.</strong> Image Sources</a></li><li class="chapter-item expanded "><a href="vmfile/resources.html"><strong aria-hidden="true">17.</strong> Resources</a></li><li class="chapter-item expanded "><a href="vmfile/network.html"><strong aria-hidden="true">18.</strong> Network Block</a></li><li class="chapter-item expanded "><a href="vmfile/cloud-init.html"><strong aria-hidden="true">19.</strong> Cloud-Init Block</a></li><li class="chapter-item expanded "><a href="vmfile/ssh.html"><strong aria-hidden="true">20.</strong> SSH Block</a></li><li class="chapter-item expanded "><a href="vmfile/provision.html"><strong aria-hidden="true">21.</strong> Provision Blocks</a></li><li class="chapter-item expanded "><a href="vmfile/multi-vm.html"><strong aria-hidden="true">22.</strong> Multi-VM Definitions</a></li><li class="chapter-item expanded "><a href="vmfile/full-example.html"><strong aria-hidden="true">23.</strong> Full Example</a></li><li class="chapter-item expanded affix "><li class="part-title">CLI Reference</li><li class="chapter-item expanded "><a href="cli/vmctl.html"><strong aria-hidden="true">24.</strong> vmctl</a></li><li class="chapter-item expanded "><a href="cli/create.html"><strong aria-hidden="true">25.</strong> vmctl create</a></li><li class="chapter-item expanded "><a href="cli/start.html"><strong aria-hidden="true">26.</strong> vmctl start</a></li><li class="chapter-item expanded "><a href="cli/stop.html"><strong aria-hidden="true">27.</strong> vmctl stop</a></li><li class="chapter-item expanded "><a href="cli/destroy.html"><strong aria-hidden="true">28.</strong> vmctl destroy</a></li><li class="chapter-item expanded "><a href="cli/list.html"><strong aria-hidden="true">29.</strong> vmctl list</a></li><li class="chapter-item expanded "><a href="cli/status.html"><strong aria-hidden="true">30.</strong> vmctl status</a></li><li class="chapter-item expanded "><a href="cli/console.html"><strong aria-hidden="true">31.</strong> vmctl console</a></li><li class="chapter-item expanded "><a href="cli/ssh.html"><strong aria-hidden="true">32.</strong> vmctl ssh</a></li><li class="chapter-item expanded "><a href="cli/suspend.html"><strong aria-hidden="true">33.</strong> vmctl suspend</a></li><li class="chapter-item expanded "><a href="cli/resume.html"><strong aria-hidden="true">34.</strong> vmctl resume</a></li><li class="chapter-item expanded "><a href="cli/image.html"><strong aria-hidden="true">35.</strong> vmctl image</a></li><li class="chapter-item expanded "><a href="cli/up.html"><strong aria-hidden="true">36.</strong> vmctl up</a></li><li class="chapter-item expanded "><a href="cli/down.html"><strong aria-hidden="true">37.</strong> vmctl down</a></li><li class="chapter-item expanded "><a href="cli/reload.html"><strong aria-hidden="true">38.</strong> vmctl reload</a></li><li class="chapter-item expanded "><a href="cli/provision.html"><strong aria-hidden="true">39.</strong> vmctl provision</a></li><li class="chapter-item expanded "><a href="cli/log.html"><strong aria-hidden="true">40.</strong> vmctl log</a></li><li class="chapter-item expanded affix "><li class="part-title">Architecture</li><li class="chapter-item expanded "><a href="architecture/overview.html"><strong aria-hidden="true">41.</strong> Overview</a></li><li class="chapter-item expanded "><a href="architecture/crate-structure.html"><strong aria-hidden="true">42.</strong> Crate Structure</a></li><li class="chapter-item expanded "><a href="architecture/backends.html"><strong aria-hidden="true">43.</strong> Hypervisor Backends</a></li><li class="chapter-item expanded "><a href="architecture/state-management.html"><strong aria-hidden="true">44.</strong> State Management</a></li><li class="chapter-item expanded "><a href="architecture/ssh.html"><strong aria-hidden="true">45.</strong> SSH Subsystem</a></li><li class="chapter-item expanded "><a href="architecture/error-handling.html"><strong aria-hidden="true">46.</strong> Error Handling</a></li><li class="chapter-item expanded affix "><li class="part-title">Library API Guide</li><li class="chapter-item expanded "><a href="library/using-as-crate.html"><strong aria-hidden="true">47.</strong> Using vm-manager as a Crate</a></li><li class="chapter-item expanded "><a href="library/hypervisor-trait.html"><strong aria-hidden="true">48.</strong> Hypervisor Trait</a></li><li class="chapter-item expanded "><a href="library/core-types.html"><strong aria-hidden="true">49.</strong> Core Types</a></li><li class="chapter-item expanded "><a href="library/image-api.html"><strong aria-hidden="true">50.</strong> Image Management API</a></li><li class="chapter-item expanded "><a href="library/ssh-provisioning-api.html"><strong aria-hidden="true">51.</strong> SSH and Provisioning API</a></li><li class="chapter-item expanded "><a href="library/vmfile-api.html"><strong aria-hidden="true">52.</strong> VMFile Parsing API</a></li><li class="chapter-item expanded affix "><li class="part-title">Advanced Topics</li><li class="chapter-item expanded "><a href="advanced/containerization.html"><strong aria-hidden="true">53.</strong> Running in Docker/Podman</a></li><li class="chapter-item expanded "><a href="advanced/tap-networking.html"><strong aria-hidden="true">54.</strong> TAP Networking and Bridges</a></li><li class="chapter-item expanded "><a href="advanced/propolis-illumos.html"><strong aria-hidden="true">55.</strong> illumos / Propolis Backend</a></li><li class="chapter-item expanded "><a href="advanced/custom-cloud-init.html"><strong aria-hidden="true">56.</strong> Custom Cloud-Init User Data</a></li><li class="chapter-item expanded "><a href="advanced/debugging.html"><strong aria-hidden="true">57.</strong> Debugging and Logs</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
