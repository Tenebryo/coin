import tensorflow as tf

with tf.name_scope('CoinNet') as scope:

    input_size = 192
    hidden_size = 256
    prior_size = 64

    #   The input to the neural network
    net_input = tf.placeholder(tf.float32, [None, input_size], name='input')

    #   Hidden layer 0
    w0 = tf.Variable(tf.random_normal([input_size,hidden_size]),name='weight_0')
    b0 = tf.Variable(tf.random_normal([hidden_size]), name='bias_0')

    hl0 = tf.nn.selu(tf.matmul(net_input, w0) + b0)

    #   Hidden layer 1
    w1 = tf.Variable(tf.random_normal([hidden_size,hidden_size]),name='weight_1')
    b1 = tf.Variable(tf.random_normal([hidden_size]), name='bias_1')

    hl1 = tf.nn.selu(tf.matmul(hl0, w1) + b1)

    #   Hidden layer 2
    w2 = tf.Variable(tf.random_normal([hidden_size,hidden_size]),name='weight_2')
    b2 = tf.Variable(tf.random_normal([hidden_size]), name='bias_2')

    hl2 = tf.nn.selu(tf.matmul(hl1, w2) + b2)

    #   Prior output layer
    wp0 = tf.Variable(tf.random_normal([hidden_size,hidden_size]),name='weight_p_0')
    bp0 = tf.Variable(tf.random_normal([hidden_size]), name='bias_p_0')

    hlp = tf.nn.selu(tf.matmul(hl2, wp0) + bp0)

    wp = tf.Variable(tf.random_normal([hidden_size,prior_size]),name='weight_p')
    bp = tf.Variable(tf.random_normal([prior_size]), name='bias_p')

    logits_p = tf.add(tf.matmul(hlp, wp), bp)
    output_p = tf.nn.softmax(logits_p, name='output_p')

    #   Value output layer
    wv0 = tf.Variable(tf.random_normal([hidden_size,hidden_size]),name='weight_v_0')
    bv0 = tf.Variable(tf.random_normal([hidden_size]), name='bias_v_0')

    hlv = tf.nn.selu(tf.matmul(hl2, wv0) + bv0)

    wv1 = tf.Variable(tf.random_normal([hidden_size,1]),name='weight_v_1')
    bv1 = tf.Variable(tf.random_normal([1]), name='bias_v_1')

    output_v = tf.tanh(tf.matmul(hlv, wv1) + bv1, name='output_v')

    #   These are the supervized learning targets to train towards
    net_target_p = tf.placeholder(tf.float32, [None, prior_size], name='target_p')
    net_target_z = tf.placeholder(tf.float32, [None, 1], name='target_z')

    #   This is the L2 regularization parameter
    l2 = tf.placeholder(tf.float32, [], name='lambda')

    #   This is the regularized loss function
    prior_loss = tf.reduce_mean(tf.nn.softmax_cross_entropy_with_logits(labels=net_target_p, logits=logits_p))
    value_loss = tf.reduce_mean(tf.squared_difference(net_target_z, output_v))
    reg_loss = tf.contrib.layers.apply_regularization(tf.contrib.layers.l2_regularizer(scale=l2), tf.trainable_variables())

    loss = tf.add(prior_loss + value_loss, reg_loss, name="loss")

    #   This is the L2 regularization parameter
    learning_rate = tf.placeholder(tf.float32, [], name='learning_rate')

    optimizer_sgd = tf.train.GradientDescentOptimizer(learning_rate).minimize(loss, name='train_sgd')
    optimizer_adm = tf.train.AdamOptimizer(learning_rate=learning_rate).minimize(loss, name='train_adm')
    optimizer_mtn = tf.train.MomentumOptimizer(learning_rate=learning_rate, momentum=0.9, use_nesterov=True).minimize(loss, name='train_mtn')

    init = tf.variables_initializer(tf.global_variables(), name = 'init')

    saver = tf.train.Saver(tf.global_variables(), name='saver')
saver_def = saver.as_saver_def()

print 'Saver Information:'

# The name of the tensor you must feed with a filename when saving/restoring.
print '  Filename: {}'.format(saver_def.filename_tensor_name)

# The name of the target operation you must run when restoring.
print '  Restore: {}'.format(saver_def.restore_op_name)

# The name of the target operation you must run when saving.
print '  Save: {}'.format(saver_def.save_tensor_name)



#   Save the 
definition = tf.Session().graph_def
directory = './data/'
tf.train.write_graph(definition, directory, 'CoinNet_model.pb', as_text=False)

# import random

# sess = tf.Session()
# sess.run([init])

# print('Loading Data...')
# file_path = './data/prior_test_data.txt'
# data = []
# data_in = []
# data_out = []
# with open(file_path, 'r') as f:
#     for ln in f.readlines():
#         s = ln.split(' ')
#         try:
#             data.append( ([float(x) for x in s[0]][:192], [float(x) for x in s[1].strip()][:64]) )
#         except Exception,e:
#             print (s)
#             print (e)
#         if random.random() > 2:
#             break
#     print('Shuffling Data...')
#     random.shuffle(data)

#     data_in = [x[0] for x in data]
#     data_out = [x[1] for x in data]

# print('Done Loading Data!')

# fd = {}
# def do_epoch():
#     global fd
#     batch_size = 128 
#     total_batch = len(data)/batch_size
#     for i in range(total_batch):
#         ix = i * batch_size
#         batch_xs = data_in[ix:ix+batch_size]
#         batch_ys = data_out[ix:ix+batch_size]

#         fd = {}
#         fd[net_input] = batch_xs
#         fd[net_target_p] = batch_ys
#         fd[net_target_z] = [[0.5]]
#         fd[learning_rate] = 0.0001
#         fd[l2] = 0.0001
#         # Run optimization op (backprop) and cost op (to get loss value)
#         l,_ = sess.run([loss, optimizer_mtn],feed_dict=fd)
#         print "Loss: {}".format(l)

# fd[net_input] = [data_in[0]]
# fd[net_target_p] = [data_out[0]]
# fd[net_target_z] = [[0.5]]
# fd[learning_rate] = 0.0001
# fd[l2] = 0.0001