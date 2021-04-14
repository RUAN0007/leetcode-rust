class LRUNode(object):
    def __init__(self, key, val): 
        self.key = key
        self.val = val
        self.next = self
        self.prev = self


class LRUCache(object):

    def __init__(self, capacity):
        """
        :type capacity: int
        """
        self.cap = capacity
        self.key2nodes = {}
        self.head = 0
        self.tail = 0
        

    def get(self, key):
        """
        :type key: int
        :rtype: int
        """
        if key in self.key2nodes:
            node = self.key2nodes[key]
            self.detach(node)
            self.push(node)
            # print("AFTER GET key={}, key2nodes={}, head.key={}, tail.key={}".format(key, self.key2nodes, self.head.key, self.tail.key))

            return node.val

        else:
            return -1
                
        

    def put(self, key, value):
        """
        :type key: int
        :type value: int
        :rtype: None
        """
        if key in self.key2nodes:
            node = self.key2nodes[key]
            node.val = value;
            self.detach(node)
            self.push(node)
        else:
            node = LRUNode(key,value)
            self.push(node)
            self.key2nodes[key] = node
        
        if len(self.key2nodes) > self.cap:
            self.pop_last()
        # print("AFTER PUT key={}, value={}, key2nodes={}, head.key={}, tail.key={}".format(key, value, self.key2nodes, self.head.key, self.tail.key))

    
    def detach(self, node):
        if len(self.key2nodes) == 1:
            self.head = 0
            self.tail = 0
            node.next = node
            node.prev = node
            del self.key2nodes[node.key]

            return
        del self.key2nodes[node.key]
       
        # prev <-> node <-> next
        # prev <-> next.
        prev = node.prev
        next = node.next
        prev.next = next
        next.prev = prev
        
        if node == self.tail:
            self.tail = prev
        if node == self.head:
            self.head = next
    
    
    def push(self, node):
        
        if self.head == 0:
            self.head = node
            self.tail = node
            node.prev = node
            node.next = node
            self.key2nodes[node.key] = node

            return
        
        # tail <-> head
        # tail <-> node <-> head
        self.tail.next = node
        node.prev = self.tail
        self.head.prev = node
        node.next = self.head
        self.head = node
        self.key2nodes[node.key] = node
    
    def pop_last(self): 
        # prev <-> tail <-> head
        # prev <-> head
        prev = self.tail.prev
        prev.next = self.head
        self.head.prev = prev
        del self.key2nodes[self.tail.key]
        self.tail = prev
        


# Your LRUCache object will be instantiated and called as such:
# obj = LRUCache(capacity)
# param_1 = obj.get(key)
# obj.put(key,value)